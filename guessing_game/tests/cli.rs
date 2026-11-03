// Integration tests that run the real guessing_game binary and talk to it
// through stdin/stdout, the same way a player would.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

#[test]
fn binary_search_wins_within_seven_guesses() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_guessing_game"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start guessing_game");

    let mut stdin = child.stdin.take().unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    let (mut low, mut high) = (1u32, 100u32);
    let mut won = false;

    // log2(100) rounded up is 7, so binary search always wins in 7 guesses.
    for _ in 0..7 {
        let guess = (low + high) / 2;
        writeln!(stdin, "{}", guess).unwrap();

        let reply = lines
            .by_ref()
            .map(|line| line.unwrap())
            .find(|line| line == "Too small!" || line == "Too big!" || line == "You win!")
            .expect("game stopped answering");

        match reply.as_str() {
            "Too small!" => low = guess + 1,
            "Too big!" => high = guess - 1,
            _ => {
                won = true;
                break;
            }
        }
    }

    assert!(won, "did not win within 7 guesses");
    let status = child.wait().unwrap();
    assert!(status.success(), "game should exit cleanly after a win");
}

#[test]
fn does_not_reveal_the_secret_number() {
    let output = Command::new(env!("CARGO_BIN_EXE_guessing_game"))
        .stdin(Stdio::null())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Guess the number!"));
    assert!(!stdout.contains("secret number"));
}

#[test]
fn non_numeric_guess_exits_with_error() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_guessing_game"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"abc\n").unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Please type a number!"));
}
