use std::fs;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn input_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    const WRONG_INPUT_FILE_NAME: &str = "test/not_existing_example.tasks.in";
    let mut cmd = Command::cargo_bin("schedule-tasks")?;

    cmd.arg(WRONG_INPUT_FILE_NAME);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Something went wrong reading the input file"));

    Ok(())
}

#[test]
fn make_schedule_for_input_file() -> Result<(), Box<dyn std::error::Error>> {
    const INPUT_FILE_NAME: &str = "./test/example.tasks.in";
    const OUTPUT_FILE_NAME: &str = "test/example.sched.out";
    const CORRECT_OUTPUT: &str = "Critical: A->B->D->H
Minimum: 4
Parallelism: 3";

    let cmd = Command::cargo_bin("schedule-tasks");

    cmd.expect("schedule-tasks binary not found")
        .arg(&INPUT_FILE_NAME)
        .assert()
        .success();

    let output_file_content = fs::read_to_string(&OUTPUT_FILE_NAME).expect("Can not read output-file");
    assert_eq!(output_file_content, CORRECT_OUTPUT);

    fs::remove_file(&OUTPUT_FILE_NAME).expect("Can not delete the test-output-file");

    Ok(())
}
