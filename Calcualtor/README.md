# Calcualtor

A command-line calculator that reads two numbers and an operator, then prints the result.

## Run

```bash
cd Calcualtor
cargo run
```

To build a standalone executable instead:

```bash
cargo build --release
```

The binary is written to `target/release/Calcualtor` (`Calcualtor.exe` on Windows).

## Command-line interface

The program takes **no command-line arguments or options**. It reads exactly three lines from standard input, in this order:

| Prompt | Expected input |
| --- | --- |
| `Please input your first number:` | A number, parsed as a 32-bit float (`f32`). Examples: `2`, `-7`, `3.5`, `1e3` |
| `Please input your second number:` | A number, same format |
| `Please input your operator [+,-,*,/]:` | One of `+`, `-`, `*`, `/` |

Leading and trailing whitespace on each line is ignored.

### Output

On success it prints one result line to standard output and exits with status `0`:

```text
welcome to the calulator
----------------------
Please input your first number: 10
Please input your second number: 4
Please input your operator [+,-,*,/]: /
The result of 10 /
 4 = 2.5
```

The line break inside the result line is real: the program prints the operator with the newline that was typed after it. This is current behavior, not a typo in this document.

### Errors and edge cases

| Situation | Behavior | Exit status |
| --- | --- | --- |
| Operator is not `+ - * /` | Prints `Unknown operator` | `0` |
| A number can't be parsed (e.g. `abc`) | Panics with a `ParseFloatError` message on stderr | non-zero (`101`) |
| Division by zero | Prints `inf`, `-inf`, or `NaN` (IEEE 754 float rules) | `0` |
| Very large or precise values | Rounded to `f32` precision (about 7 significant digits) | `0` |

### Scripted use

Since all input comes from stdin, you can pipe it in:

```bash
printf '2\n3\n+\n' | cargo run -q
```

## Tests

```bash
cargo test
```

[`tests/cli.rs`](tests/cli.rs) runs the real binary with piped input and checks addition, subtraction, multiplication, division, division by zero, unknown operators, and non-numeric input.
