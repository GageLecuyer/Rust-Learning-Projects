# guessing_game

A command-line number guessing game, based on the guessing game in *The Rust Programming Language* book. The program picks a secret whole number from 1 to 100 and you guess until you get it.

## Run

```bash
cd guessing_game
cargo run
```

To build a standalone executable instead:

```bash
cargo build --release
```

The binary is written to `target/release/guessing_game` (`guessing_game.exe` on Windows).

## Command-line interface

The program takes **no command-line arguments or options**. All input comes from standard input, one guess per line.

| Input | Meaning |
| --- | --- |
| A whole number (e.g. `42`) | Your guess. Parsed as an unsigned 32-bit integer (`u32`). Surrounding whitespace is ignored. |

### Output

After each guess the program prints `You guessed: N` and then one of:

| Message | Meaning |
| --- | --- |
| `Too small!` | The secret number is higher. Guess again. |
| `Too big!` | The secret number is lower. Guess again. |
| `You win!` | Correct. It then prints `You guessed N many times` and exits with status `0`. |

Example session:

```text
Guess the number!

Please input your guess.
50
You guessed: 50
Too big!

Please input your guess.
25
You guessed: 25
Too small!

Please input your guess.
37
You guessed: 37
You win!
You guessed 3 many times
```

### Errors

| Situation | Behavior | Exit status |
| --- | --- | --- |
| Input isn't a non-negative whole number (e.g. `abc`, `-5`, `4.2`) | Panics with `Please type a number!` on stderr | non-zero (`101`) |
| Standard input closes (end of file) before you win | The empty line can't be parsed, so it panics with `Please type a number!` | non-zero (`101`) |

Numbers outside 1 to 100 are accepted as guesses and get `Too small!` or `Too big!`.

## Dependencies

- [`rand`](https://crates.io/crates/rand) 0.10, used to pick the secret number. This is for gameplay only, not for anything security-related.

## Tests

```bash
cargo test
```

[`tests/cli.rs`](tests/cli.rs) runs the real binary. One test plays a full game using binary search and checks that it wins within 7 guesses and then exits cleanly. Others check that the secret number is never printed and that non-numeric input is rejected.
