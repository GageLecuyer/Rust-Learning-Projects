// Integration tests that run the real Calcualtor binary with piped input
// and check what it prints.

use std::io::Write;
use std::process::{Command, Output, Stdio};

/// Runs the calculator, typing `input` into stdin, and returns its output.
fn run(input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_Calcualtor"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start Calcualtor");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn adds() {
    let out = run("2\n3\n+\n");
    assert!(out.status.success());
    assert!(stdout(&out).contains("= 5"));
}

#[test]
fn subtracts() {
    let out = run("10\n4\n-\n");
    assert!(out.status.success());
    assert!(stdout(&out).contains("= 6"));
}

#[test]
fn multiplies() {
    let out = run("1.5\n4\n*\n");
    assert!(out.status.success());
    assert!(stdout(&out).contains("= 6"));
}

#[test]
fn divides() {
    let out = run("10\n4\n/\n");
    assert!(out.status.success());
    assert!(stdout(&out).contains("= 2.5"));
}

#[test]
fn divide_by_zero_gives_infinity() {
    let out = run("1\n0\n/\n");
    assert!(out.status.success());
    assert!(stdout(&out).contains("= inf"));
}

#[test]
fn rejects_unknown_operator() {
    let out = run("1\n2\n%\n");
    assert!(out.status.success());
    assert!(stdout(&out).contains("Unknown operator"));
}

#[test]
fn non_numeric_input_exits_with_error() {
    let out = run("abc\n2\n+\n");
    assert!(!out.status.success());
}
