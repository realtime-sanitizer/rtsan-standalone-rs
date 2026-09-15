"""Run with: python3 -m unittest discover -s tests/assembly -v."""

import unittest

import check


def function(name, body, marker=False):
    end = ".Lfunc_end0:\n" if marker else ""
    return f".type {name},@function\n{name}:\n.cfi_startproc\n{body}\n{end}.size {name}, .-{name}\n"


def fixture(arch="x86_64"):
    body = "leaq 1(%rdi), %rax\nretq" if arch == "x86_64" else "add x0, x0, #1\nret"
    result = function("baseline", body) + function("empty_baseline", "ret")
    for baseline, cases in check.CASES.items():
        result += "".join(f".set {name}, {baseline}\n" for name in cases)
    return result


class AssemblyTests(unittest.TestCase):
    def test_all_eight_cases_and_architectures(self):
        self.assertEqual(sum(map(len, check.CASES.values())), 8)
        for arch in ("x86_64", "aarch64"):
            with self.subTest(arch=arch):
                check.check_assembly(fixture(arch), arch)

    def test_separate_bodies_ignore_metadata_and_whitespace(self):
        text = fixture().replace(".set my_nonblocking, baseline\n", "")
        text += function("my_nonblocking", "\tleaq\t1(%rdi), %rax # increment\n\nretq", True)
        check.check_assembly(text, "x86_64")

    def test_alias_chain_forward_reference(self):
        text = fixture().replace(".set my_blocking, baseline", ".set my_blocking, other")
        check.check_assembly(text + ".set other, baseline\n", "x86_64")

    def test_assignment_aliases(self):
        for arch in ("x86_64", "aarch64"):
            text = fixture(arch)
            for baseline, cases in check.CASES.items():
                for name in cases:
                    text = text.replace(f".set {name}, {baseline}", f"{name} = {baseline}")
            with self.subTest(arch=arch):
                check.check_assembly(text, arch)

    def test_mixed_alias_chain(self):
        text = fixture().replace(".set my_blocking, baseline", "my_blocking = other")
        check.check_assembly(text + ".set other, baseline\n", "x86_64")

    def test_assignment_alias_failures(self):
        for assignment, extra, error in (
            ("my_blocking = absent", "", "missing function symbol: absent"),
            ("my_blocking = other", ".set other, my_blocking\n", "alias cycle"),
            ("my_blocking = my_blocking", "", "alias cycle"),
            ("my_blocking = baseline", "my_blocking = baseline\n", "ambiguous alias"),
            ("my_blocking = baseline + 1", "", "missing function symbol: my_blocking"),
        ):
            text = fixture().replace(".set my_blocking, baseline", assignment) + extra
            with self.subTest(assignment=assignment, extra=extra), self.assertRaisesRegex(check.CheckError, error):
                check.check_assembly(text, "x86_64")

    def test_alias_cycle(self):
        text = fixture().replace(".set my_blocking, baseline", ".set my_blocking, other")
        with self.assertRaisesRegex(check.CheckError, "alias cycle"):
            check.check_assembly(text + ".set other, my_blocking\n", "x86_64")

    def test_missing_each_case(self):
        for baseline, cases in check.CASES.items():
            for name in cases:
                with self.subTest(name=name), self.assertRaisesRegex(check.CheckError, "missing function"):
                    check.check_assembly(fixture().replace(f".set {name}, {baseline}\n", ""), "x86_64")

    def test_missing_alias_target(self):
        with self.assertRaisesRegex(check.CheckError, "missing function symbol: absent"):
            check.check_assembly(fixture().replace("my_blocking, baseline", "my_blocking, absent"), "x86_64")

    def test_missing_baselines(self):
        for name in check.CASES:
            with self.subTest(name=name), self.assertRaisesRegex(check.CheckError, "missing function"):
                check.check_assembly(fixture().replace(f"\n{name}:\n", f"\nrenamed_{name}:\n")
                                     .replace(f".size {name},", f".size renamed_{name},"), "x86_64")

    def test_empty_input_cannot_pass(self):
        with self.assertRaisesRegex(check.CheckError, "missing function"):
            check.check_assembly("", "x86_64")

    def test_empty_baseline_bodies(self):
        for name in check.CASES:
            with self.subTest(name=name), self.assertRaisesRegex(check.CheckError, "has no instructions"):
                check.check_assembly(function(name, "") + (
                    function("baseline", "ret") if name == "empty_baseline" else ""
                ) + "".join(f".set {case}, baseline\n" for case in check.CASES["baseline"]), "x86_64")

    def test_extra_instruction_negative_control(self):
        for baseline, cases in check.CASES.items():
            for name in cases:
                text = fixture().replace(f".set {name}, {baseline}\n", "")
                body = "leaq 1(%rdi), %rax\n" if baseline == "baseline" else ""
                text += function(name, body + "nop\n" + ("retq" if baseline == "baseline" else "ret"))
                with self.subTest(name=name), self.assertRaisesRegex(check.CheckError, "differs from"):
                    check.check_assembly(text, "x86_64")

    def test_empty_baseline_must_be_only_return(self):
        with self.assertRaisesRegex(check.CheckError, "must be only a return"):
            check.check_assembly(fixture().replace("empty_baseline:\n", "empty_baseline:\nnop\n"), "x86_64")

    def test_internal_branch_label_rejected(self):
        with self.assertRaisesRegex(check.CheckError, "internal label"):
            check.parse_assembly(function("baseline", "jmp .Lwork\n.Lwork:\nret"), "x86_64")

    def test_instruction_after_end_marker_rejected(self):
        with self.assertRaisesRegex(check.CheckError, "after end marker"):
            check.parse_assembly(function("baseline", "ret\n.Lfunc_end0:\nnop"), "x86_64")

    def test_embedded_instruction_directive_rejected(self):
        with self.assertRaisesRegex(check.CheckError, "embedded"):
            check.parse_assembly(function("baseline", ".inst 0xd503201f\nret"), "aarch64")

    def test_alignment_and_unknown_directives_rejected(self):
        for directive in (".p2align 4", ".balign 16", ".unknown"):
            with self.subTest(directive=directive), self.assertRaisesRegex(check.CheckError, "embedded"):
                check.parse_assembly(function("baseline", directive + "\nret"), "x86_64")

    def test_missing_size_rejected(self):
        with self.assertRaisesRegex(check.CheckError, "missing .size"):
            check.parse_assembly("baseline:\nret\n", "x86_64")

    def test_duplicate_symbol_rejected(self):
        with self.assertRaisesRegex(check.CheckError, "duplicate symbol"):
            check.parse_assembly(function("baseline", "ret") * 2, "x86_64")

    def test_host_support(self):
        for arch in ("x86_64", "aarch64"):
            self.assertEqual(check.host_arch(f"rustc 1.90\nhost: {arch}-unknown-linux-gnu\n"), arch)
        for host in ("x86_64-apple-darwin", "aarch64-apple-darwin", "i686-unknown-linux-gnu", ""):
            with self.subTest(host=host), self.assertRaisesRegex(check.CheckError, "unsupported host"):
                check.host_arch(f"host: {host}\n")


if __name__ == "__main__":
    unittest.main()
