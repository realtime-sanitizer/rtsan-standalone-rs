#!/usr/bin/env python3
"""Check that disabled RTSan macros produce baseline optimized ELF assembly."""

import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile


CASES = {
    "baseline": (
        "my_nonblocking", "my_blocking", "my_no_sanitize", "my_scoped_disabler",
    ),
    "empty_baseline": (
        "empty_nonblocking", "empty_blocking", "empty_no_sanitize",
        "empty_scoped_disabler",
    ),
}
SYMBOL = r"[A-Za-z_.$][A-Za-z0-9_.$]*"


class CheckError(Exception):
    pass


def host_arch(version):
    host = next((line[6:].strip() for line in version.splitlines()
                 if line.startswith("host: ")), "")
    parts = host.split("-")
    if not parts or parts[0] not in ("x86_64", "aarch64") or "linux" not in parts[1:]:
        raise CheckError(f"unsupported host {host!r}: need Linux x86_64 or aarch64")
    return parts[0]


def parse_assembly(text, arch):
    """Return instructions and .set/assignment aliases; fail on ambiguous bodies."""
    if arch not in ("x86_64", "aarch64"):
        raise CheckError(f"unsupported architecture: {arch}")
    functions, aliases = {}, {}
    current, instructions, ended = None, [], False
    for raw in text.splitlines():
        line = raw.split("//", 1)[0]
        if arch == "x86_64":
            line = line.split("#", 1)[0]
        line = line.strip()
        if not line:
            continue
        alias = (re.fullmatch(rf"\.set\s+({SYMBOL})\s*,\s*({SYMBOL})", line)
                 or re.fullmatch(rf"({SYMBOL})\s*=\s*({SYMBOL})", line))
        if alias:
            name, target = alias.groups()
            if current is not None or name in aliases or name in functions:
                raise CheckError(f"ambiguous alias: {line}")
            aliases[name] = target
            continue
        size = re.match(rf"\.size\s+({SYMBOL})\s*,", line)
        if size:
            if current is not None:
                if size[1] != current:
                    raise CheckError(f"wrong .size for {current}: {line}")
                functions[current] = tuple(instructions)
                current = None
            continue
        label = re.fullmatch(rf"({SYMBOL}):", line)
        if label:
            name = label[1]
            if current is None:
                if name.startswith(".L"):
                    continue
                if name in functions or name in aliases:
                    raise CheckError(f"duplicate symbol: {name}")
                current, instructions, ended = name, [], False
            elif re.fullmatch(r"\.Lfunc_end\d+", name) and not ended:
                ended = True  # LLVM's end marker is metadata, not a branch target.
            else:
                raise CheckError(f"unexpected internal label in {current}: {name}")
            continue
        if current is None:
            continue
        if line.startswith("."):
            directive = line.split()[0]
            if directive.startswith(".cfi_") or directive in {".loc", ".file"}:
                continue
            raise CheckError(f"embedded data/instruction directive in {current}: {line}")
        if ended:
            raise CheckError(f"instruction after end marker in {current}: {line}")
        instructions.append(" ".join(line.split()))
    if current is not None:
        raise CheckError(f"missing .size for {current}")
    return functions, aliases


def resolve(name, functions, aliases):
    seen = set()
    while name in aliases:
        if name in seen:
            raise CheckError(f"alias cycle at {name}")
        seen.add(name)
        name = aliases[name]
    if name not in functions:
        raise CheckError(f"missing function symbol: {name}")
    return functions[name]


def check_assembly(text, arch):
    functions, aliases = parse_assembly(text, arch)
    for baseline, cases in CASES.items():
        expected = resolve(baseline, functions, aliases)
        if not expected:
            raise CheckError(f"baseline {baseline} has no instructions")
        if baseline == "empty_baseline":
            returns = {("ret",), ("retq",)} if arch == "x86_64" else {("ret",)}
            if expected not in returns:
                raise CheckError(f"empty_baseline must be only a return, got {expected!r}")
        for name in cases:
            actual = resolve(name, functions, aliases)
            if actual != expected:
                raise CheckError(
                    f"{name} differs from {baseline}:\n"
                    f"  expected: {expected!r}\n  actual:   {actual!r}"
                )


def run(command, root, env):
    try:
        result = subprocess.run(command, cwd=root, env=env, text=True,
                                capture_output=True, check=True)
    except OSError as error:
        raise CheckError(f"cannot run {command[0]}: {error}") from error
    except subprocess.CalledProcessError as error:
        raise CheckError(
            f"command failed ({error.returncode}): {' '.join(command)}\n"
            f"{error.stderr}\n{error.stdout}"
        ) from error
    return result.stdout


def main():
    root = Path(__file__).resolve().parents[2]
    env = dict(os.environ, RTSAN_ENABLE="0")
    try:
        arch = host_arch(run(["rustc", "-vV"], root, env))
        fixture = Path(__file__).with_name("fixture.rs").resolve()
        if not fixture.is_file():
            raise CheckError(f"missing assembly fixture: {fixture}")
        output = run(["cargo", "build", "--locked", "--release", "--lib", "--message-format=json"],
                     root, env)
        rlibs, dependency_dirs = set(), set()
        for line in output.splitlines():
            try:
                message = json.loads(line)
            except json.JSONDecodeError as error:
                raise CheckError(f"invalid Cargo JSON output: {line}") from error
            if (message.get("reason") == "compiler-artifact"
                    and message.get("target", {}).get("name") == "rtsan_standalone"):
                filenames = [Path(name) for name in message.get("filenames", [])]
                rlibs.update(path for path in filenames if path.suffix == ".rlib")
                dependency_dirs.update(path.parent for path in filenames
                                       if path.suffix == ".rmeta")
        if len(rlibs) != 1:
            raise CheckError(f"expected one rtsan_standalone rlib, found: {sorted(map(str, rlibs))}")
        if len(dependency_dirs) != 1:
            raise CheckError(f"expected one dependency directory, found: {dependency_dirs}")
        dependencies = next(iter(dependency_dirs))
        if not dependencies.is_absolute():
            dependencies = root / dependencies
        if not dependencies.is_dir():
            raise CheckError(f"missing dependency directory: {dependencies}")
        rlib = next(iter(rlibs))
        if not rlib.is_absolute():
            rlib = root / rlib
        if not rlib.is_file():
            raise CheckError(f"Cargo reported missing rlib: {rlib}")
        with tempfile.TemporaryDirectory(prefix="rtsan-assembly-") as temporary:
            assembly = Path(temporary) / "fixture.s"
            run(["rustc", str(fixture), "--edition=2021", "--crate-type=lib",
                 "-C", "opt-level=3", "--emit=asm", "--extern",
                 f"rtsan_standalone={rlib}", "-L", f"dependency={dependencies}",
                 "-o", str(assembly)], root, env)
            check_assembly(assembly.read_text(), arch)
    except (CheckError, OSError) as error:
        print(f"assembly check failed: {error}", file=sys.stderr)
        return 1
    print("Assembly check passed: all 8 disabled macro cases match their baselines.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
