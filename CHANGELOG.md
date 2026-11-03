# Changelog

Notable changes to this repository are listed here. The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and project versions follow [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- MIT `LICENSE` file, plus `license = "MIT"` in each `Cargo.toml`.
- `CONTRIBUTING.md`, `SECURITY.md`, and this changelog.
- A README for each project describing its command-line or keyboard interface.
- Automated tests:
  - integration tests for `Calcualtor` and `guessing_game` in their `tests/` folders
  - unit tests for snake logic in `snake_game/src/snake.rs`
- GitHub Actions CI that runs `cargo fmt --check`, `cargo clippy -D warnings`, and `cargo test` for every project, plus a `cargo audit` dependency vulnerability scan.
- Dependabot configuration for Cargo dependencies and GitHub Actions.
- Pull request template (`.github/pull_request_template.md`) and a documented code review process in `CONTRIBUTING.md`.

### Fixed

- `guessing_game` printed the secret number at the start of every game.
- `guessing_game` never exited after a correct guess. It now exits after `You win!`.

### Changed

- `snake_game` dependencies now use version requirements (`rand = "0.10"`, `piston_window = "0.148"`) instead of `"*"`. The locked versions are unchanged.
- Source code formatted with `rustfmt`, and two `clippy` warnings fixed: an unused import in `guessing_game` and a collapsible `if` in `snake_game`. No behavior change.

## Earlier history

Before this changelog existed, the repository had three projects: `guessing_game`, `Calcualtor`, and `snake_game`. See the [commit history](https://github.com/GageLecuyer/Rust-Learning-Projects/commits/Main) for details.
