extern crate libtest_mimic;

use libtest_mimic::{Arguments, Failed, Trial};
use regex::bytes::Regex;
use std::{
    fs::read_dir,
    process::{Command, ExitCode, Stdio},
};

fn main() -> ExitCode {
    let args = Arguments::from_args();

    let mut test_cases: Vec<(String, Regex)> = Vec::new();

    // collect test cases
    for file in read_dir("detection-tests/examples").unwrap() {
        let file = file.unwrap();
        // println!("{:?}", file);
        assert!(file.metadata().unwrap().is_file());
        let content = std::fs::read_to_string(file.path()).unwrap();
        let first_line = content.lines().next().unwrap().strip_prefix("// ").unwrap();
        let regex = Regex::new(first_line).unwrap();

        test_cases.push((file.file_name().into_string().unwrap().strip_suffix(".rs").unwrap().to_string(), regex));
    }

    // println!("{test_cases:?}");
    // create tests
    let tests = test_cases
        .into_iter()
        .map(|(name, regex)| {
            Trial::test(name.clone(), move || {
                let process = Command::new("cargo")
                    .args([
                        "run".to_owned(),
                        "--package".to_owned(),
                        "detection-tests".to_owned(),
                        "--example".to_owned(),
                        name,
                        "--features".to_owned(),
                        "rtsan".to_owned(),
                    ])
                    .stderr(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();
                let output = process.wait_with_output().unwrap();
                // println!("{output:?}");
                if output.status.success() {
                    Err(Failed::from("no violation detected"))
                } else if regex.is_match(&output.stderr) {
                    Ok(())
                } else {
                    Err(Failed::from("stderr didn't match regex"))
                }
            })
        })
        .collect();

    // println!("{tests:?}");

    libtest_mimic::run(&args, tests).exit_code()
}
