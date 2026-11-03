# snake_game

A classic snake game in a desktop window, built with [`piston_window`](https://crates.io/crates/piston_window). Steer the snake to eat food and grow without hitting the walls or yourself.

## Run

```bash
cd snake_game
cargo run --release
```

The first build downloads and compiles the graphics dependencies, which can take a few minutes. `--release` is optional, but the game runs more smoothly with it.

### Requirements

- A desktop session with a display. It won't run over SSH without a display or in a headless container.
- Working graphics drivers.
- On Linux, an X11 or Wayland session.

## Interface

The program takes **no command-line arguments or options**. It opens a 500×500 window titled **Snake Game**.

### Controls

| Key | Action |
| --- | --- |
| `↑` or `W` | Move up |
| `↓` or `S` | Move down |
| `←` or `A` | Move left |
| `→` or `D` | Move right |
| `R` | Restart (only after game over) |
| `Esc` | Quit |

You can't reverse straight into yourself: pressing the opposite of your current direction does nothing.

### Rules

- The board is 20×20 cells. The snake starts 3 cells long, moving right, and moves 8 cells per second.
- Eating the red food adds 1 to your score and 1 cell to the snake's length.
- Hitting a wall or the snake's own body ends the game. The background turns red.
- Filling the whole board wins the game.

### Output

| Where | What |
| --- | --- |
| Window title | `Snake Game \| Score: N` while playing, `Game Over! Score: N \| Press R to Restart` on a loss, `You Win! Score: N \| Press R to Restart` on a win |
| Standard output | `Score: N` each time the score goes up |

Closing the window or pressing `Esc` exits with status `0`. If no window can be created (for example, no display is available), the program panics and exits with a non-zero status.

## Code layout

| File | Purpose |
| --- | --- |
| [`src/main.rs`](src/main.rs) | Window setup, game loop, keyboard handling, food placement |
| [`src/snake.rs`](src/snake.rs) | `Snake` type: movement, direction changes, collisions, growth |
| [`src/draw.rs`](src/draw.rs) | Draws one grid cell |

## Dependencies

- [`piston_window`](https://crates.io/crates/piston_window) 0.148, for the window, input, and 2D drawing.
- [`rand`](https://crates.io/crates/rand) 0.10, to place food at random. This is for gameplay only, not for anything security-related.

## Tests

```bash
cargo test
```

Unit tests in [`src/snake.rs`](src/snake.rs) check the starting snake, default movement, growth on eating, wall and self collisions, turning, and blocking reversals. They test game logic only and don't open a window, so they also run in CI.
