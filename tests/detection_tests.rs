extern crate libtest_mimic;

use libtest_mimic::{Arguments, Failed, Trial};
use std::{
    fs::read_dir,
    process::{Command, ExitCode, Stdio},
};

fn main() -> ExitCode {
    let args = Arguments::from_args();

    let mut test_cases: Vec<String> = Vec::new();

    // collect test cases
    for file in read_dir("tests/detection-tests/src/bin").unwrap() {
        let file = file.unwrap();

        assert!(file.metadata().unwrap().is_file());

        test_cases.push(
            file.file_name()
                .to_str()
                .unwrap()
                .strip_suffix(".rs")
                .unwrap()
                .to_owned(),
        );
    }

    let ignored = !cfg!(rtsan_supported);

    if ignored {
        println!("WARNING: RTSAN not supported. Skipping detection tests");
    }

    // run tests
    let tests = test_cases
        .into_iter()
        .map(|name| {
            Trial::test(name.clone(), move || {
                let process = Command::new("cargo")
                    .args(["run", "-p", "detection-tests", "--bin", &name])
                    .env("RTSAN_ENABLE", "1")
                    .stderr(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();
                let output = process.wait_with_output().unwrap();

                if output.status.success() {
                    Err(Failed::from("no violation detected."))
                } else {
                    Ok(())
                }
            })
            .with_ignored_flag(ignored)
        })
        .collect();

    libtest_mimic::run(&args, tests).exit_code()
}
