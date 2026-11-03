# Contributing to Rust-Learning-Projects

Thanks for your interest. This is a personal learning repository, but bug reports, suggestions, and pull requests are welcome.

## Reporting problems

- **Bugs and enhancement requests:** open an issue at <https://github.com/GageLecuyer/Rust-Learning-Projects/issues>. Include:
  - which project it's about (`Calcualtor`, `guessing_game`, or `snake_game`)
  - what you did (the exact input or keys), what you expected, and what happened
  - your operating system and the output of `rustc --version`
- **Security vulnerabilities:** do **not** use public issues. Report them privately as described in [SECURITY.md](SECURITY.md).

I aim to respond to new issues within 14 days.

## Making changes

1. Fork the repository and create a branch from `Main`:

   ```bash
   git checkout -b my-change Main
   ```

2. Make your change inside the relevant project folder. Each project is a separate Cargo package, so run Cargo commands from that folder (for example `cd guessing_game`).
3. Keep each pull request focused on one change.

## Rust coding conventions

- **Formatting:** code must be formatted with `rustfmt` using the default settings:

  ```bash
  cargo fmt
  ```

- **Linting:** code must have no `clippy` warnings:

  ```bash
  cargo clippy --all-targets --locked -- -D warnings
  ```

- **Naming:** follow the standard Rust naming conventions. Use `snake_case` for functions, variables, and modules, `UpperCamelCase` for types and enum variants, and `SCREAMING_SNAKE_CASE` for constants.
- **Errors:** prefer clear `expect("...")` messages over bare `unwrap()` in new code, so failures explain themselves.
- **No `unsafe` code** unless it's truly necessary and explained in a comment.
- **Dependencies:** only add a dependency when it's clearly needed. Use a specific version requirement (e.g. `rand = "0.10"`, not `"*"`) and commit the updated `Cargo.lock`.
- **Edition:** projects use Rust 2024 edition. Keep new projects on the same edition.

## Tests

**Policy: any new functionality must come with automated tests, and bug fixes should include a test that would have caught the bug.** Pull requests that add features without tests may be asked to add them before merging.

- Unit tests go in a `#[cfg(test)] mod tests` block in the same file as the code (see `snake_game/src/snake.rs`).
- Tests for command-line behavior go in the project's `tests/` folder and run the real binary (see `guessing_game/tests/cli.rs`).

Run the tests with:

```bash
cargo test --locked
```

## Submitting a pull request

1. Before you push, run these in every project you changed:

   ```bash
   cargo fmt --check
   ```

   ```bash
   cargo clippy --all-targets --locked -- -D warnings
   ```

   ```bash
   cargo test --locked
   ```

2. Push your branch and open a pull request against `Main` at <https://github.com/GageLecuyer/Rust-Learning-Projects/pulls>.
3. In the description, explain what changed and why, and link any related issue (e.g. `Fixes #12`).
4. GitHub Actions runs the same checks automatically. A pull request needs passing CI before it can be merged.
5. If you add a new project or change a program's inputs or outputs, update that project's `README.md` and the table in the top-level `README.md`.
6. Add a line under **Unreleased** in [CHANGELOG.md](CHANGELOG.md) for any user-visible change.

## License of contributions

By submitting a contribution, you agree that it's licensed under this repository's [MIT License](LICENSE).
