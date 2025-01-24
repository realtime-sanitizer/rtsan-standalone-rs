extern crate libtest_mimic;

use std::process::{Command, ExitCode, Stdio};
use libtest_mimic::{Arguments, Trial, Failed};

fn main() -> ExitCode {
    let args = Arguments::from_args();

    let test_names = ["test_detection"];

    let tests = test_names.iter().map(|name| {
        Trial::test(name.to_string(), || {
            let status = Command::new("cargo").args(["test", name, "--features", "enable", "--", "--ignored"]).stderr(Stdio::null()).status().unwrap();
            if status.success() {
                Err(Failed::without_message())
            } else {
                Ok(())
            }
        })
    }).collect();

    libtest_mimic::run(&args, tests).exit_code()
}
