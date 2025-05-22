extern crate libtest_mimic;

use libtest_mimic::{Arguments, Failed, Trial};
use std::{
    fs::{read_dir, File},
    io::{BufRead, BufReader},
    process::{Command, ExitCode, Stdio},
};

#[derive(Debug)]
struct Test {
    name: String,
    checks: Vec<String>,
}

fn main() -> ExitCode {
    let args = Arguments::from_args();

    let mut test_cases: Vec<Test> = Vec::new();

    // collect test cases
    for file in read_dir("tests/detection-tests/src/bin").unwrap() {
        let file = file.unwrap();

        assert!(file.metadata().unwrap().is_file());

        let reader = BufReader::new(File::open(file.path()).unwrap());
        let checks = reader
            .lines()
            .map_while(|line| {
                line.unwrap()
                    .strip_prefix("// check: ")
                    .map(|str| str.to_owned())
            })
            .collect();

        let name = file
            .file_name()
            .to_str()
            .unwrap()
            .strip_suffix(".rs")
            .unwrap()
            .to_owned();

        test_cases.push(Test { name, checks });
    }

    let ignored = !cfg!(rtsan_supported);

    if ignored {
        println!("WARNING: RTSAN not supported. Skipping detection tests");
    }

    // run tests
    let tests = test_cases
        .into_iter()
        .map(|test| {
            Trial::test(test.name.clone(), move || {
                let process = Command::new("cargo")
                    .args(["run", "-p", "detection-tests", "--bin", &test.name])
                    .env("RTSAN_ENABLE", "1")
                    .stderr(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();
                let output = process.wait_with_output().unwrap();

                if output.status.success() {
                    Err(Failed::from("no violation detected."))
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    for check in &test.checks {
                        if !stderr.contains(check) {
                            return Err(Failed::from(
                                String::from("wrong detection output. expected: \n")
                                    + check
                                    + "\ngot:\n"
                                    + &stderr,
                            ));
                        }
                    }
                    Ok(())
                }
            })
            .with_ignored_flag(ignored)
        })
        .collect();

    libtest_mimic::run(&args, tests).exit_code()
}
