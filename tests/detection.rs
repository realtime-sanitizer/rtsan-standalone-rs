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
    for file in read_dir("tests/detection/examples").unwrap() {
        let file = file.unwrap();
        // println!("{:?}", file);
        assert!(file.metadata().unwrap().is_file());

        test_cases.push(
            file.file_name()
                .into_string()
                .unwrap()
                .strip_suffix(".rs")
                .unwrap()
                .to_string(),
        );
    }

    // println!("{test_cases:?}");
    // create tests
    let tests = test_cases
        .into_iter()
        .map(|name| {
            Trial::test(name.clone(), move || {
                let process = Command::new("cargo")
                    .args([
                        "run".to_owned(),
                        "--example".to_owned(),
                        name,
                        "--features".to_owned(),
                        "rtsan".to_owned(),
                    ])
                    .stderr(Stdio::piped())
                    .stdout(Stdio::piped())
                    .current_dir("tests/detection/")
                    .spawn()
                    .unwrap();
                let output = process.wait_with_output().unwrap();
                // println!("{output:?}");
                if output.status.success() {
                    Err(Failed::from(format!(
                        "no violation detected. output: {output:?}"
                    )))
                } else {
                    Ok(())
                }
            })
        })
        .collect();

    // println!("{tests:?}");

    libtest_mimic::run(&args, tests).exit_code()
}
