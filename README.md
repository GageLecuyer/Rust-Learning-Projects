[![CI](https://github.com/GageLecuyer/Rust-Learning-Projects/actions/workflows/ci.yml/badge.svg?branch=Main)](https://github.com/GageLecuyer/Rust-Learning-Projects/actions/workflows/ci.yml)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/GageLecuyer/Rust-Learning-Projects/badge)](https://scorecard.dev/viewer/?uri=github.com/GageLecuyer/Rust-Learning-Projects)
[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/16313/badge)](https://www.bestpractices.dev/projects/16313)

# Rust-Learning-Projects

A collection of Rust projects, exercises, and small programs created while learning the Rust programming language. This repository documents my progress with Rust fundamentals, ownership and borrowing, structs, enums, error handling, collections, modules, and other core concepts through hands-on practice.

The programs are small and meant for learning. They are not intended for production use.

## Projects

Each project is its own independent Cargo package in its own folder. There is no top-level Cargo workspace, so you run Cargo commands from inside a project folder.

| Folder | What it is | Interface | Docs |
| --- | --- | --- | --- |
| [`Calcualtor/`](Calcualtor/) | Four-function calculator for two numbers | Command line (stdin/stdout) | [Calcualtor/README.md](Calcualtor/README.md) |
| [`guessing_game/`](guessing_game/) | Guess a random number from 1 to 100 | Command line (stdin/stdout) | [guessing_game/README.md](guessing_game/README.md) |
| [`snake_game/`](snake_game/) | Classic snake game in a desktop window | Graphical window + keyboard | [snake_game/README.md](snake_game/README.md) |

(The calculator folder and package name are spelled `Calcualtor`. The spelling is kept so existing links and build commands keep working.)

## Requirements

- **Rust 1.85 or newer** (all projects use the Rust 2024 edition). Install Rust with [rustup](https://rustup.rs/), which also installs `cargo`.
- **Git**, to clone the repository.
- For `snake_game` only: a desktop session with a display (Windows, macOS, or Linux with X11 or Wayland) and working graphics drivers.

Check your toolchain:

```bash
rustc --version
cargo --version
```

## Installation

```bash
git clone https://github.com/GageLecuyer/Rust-Learning-Projects.git
cd Rust-Learning-Projects
```

Nothing else needs installing. Cargo downloads each project's dependencies (pinned in that project's `Cargo.lock`) the first time you build it.

## Running a project

Change into the project's folder and use `cargo run`:

```bash
cd Calcualtor
cargo run
```

```bash
cd guessing_game
cargo run
```

```bash
cd snake_game
cargo run --release
```

### Inputs and outputs at a glance

| Project | Input | Output |
| --- | --- | --- |
| Calcualtor | Three lines on stdin: first number, second number, operator (`+`, `-`, `*`, `/`) | The result, e.g. `The result of 2 + 3 = 5`, or `Unknown operator` |
| guessing_game | One whole number (1 to 100) per line on stdin, repeated until correct | `Too small!`, `Too big!`, or `You win!` plus the number of guesses |
| snake_game | Arrow keys or W/A/S/D to steer, R to restart after game over, Esc to quit | A 500×500 game window; the score in the window title and `Score: N` on stdout |

Each project's README covers its full command-line or keyboard interface, example sessions, and error behavior.

## Testing

Every project has automated tests that run with the standard `cargo test` command:

```bash
cd Calcualtor && cargo test
```

```bash
cd guessing_game && cargo test
```

```bash
cd snake_game && cargo test
```

- `Calcualtor/tests/cli.rs` and `guessing_game/tests/cli.rs` are integration tests. They start the real program, type input into it, and check what it prints.
- `snake_game/src/snake.rs` has unit tests for snake movement, growth, turning, and collisions. They don't open a window.

The same checks run automatically on GitHub Actions for every push and pull request (see [`.github/workflows/ci.yml`](.github/workflows/ci.yml)):

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

## Reporting bugs and requesting enhancements

- **Bugs and feature requests:** open an issue at <https://github.com/GageLecuyer/Rust-Learning-Projects/issues>. Say which project it's about, what you did, what you expected, and what happened instead. Include your OS and the output of `rustc --version`.
- **Security vulnerabilities:** do **not** open a public issue. Follow [SECURITY.md](SECURITY.md) instead.
- **Contributing code:** see [CONTRIBUTING.md](CONTRIBUTING.md).

## Changes

See [CHANGELOG.md](CHANGELOG.md) for notable changes.

## License

This project is licensed under the [MIT License](LICENSE).
