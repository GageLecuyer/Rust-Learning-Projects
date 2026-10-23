mod snake;
mod draw;

use piston_window::*;
use piston_window::graphics::clear;
use snake::{Snake, Direction};
use draw::draw_block;

const WIDTH: i32 = 20;
const HEIGHT: i32 = 20;

const FOOD_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

// Create food at a random empty position
fn random_food(snake: &Snake) -> Option<(i32, i32)> {

    let mut empty_positions = Vec::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {

            if !snake.contains(x, y) {
                empty_positions.push((x, y));
            }
        }
    }

    if empty_positions.is_empty() {
        return None;
    }

    let index = rand::random_range(0..empty_positions.len());

    Some(empty_positions[index])
}

fn main() {

    // Create game window
    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [500, 500])
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Snake moves 8 times per second
    window.set_ups(8);

    // Create snake
    let mut snake = Snake::new(5, 10);

    // Create food
    let mut food = random_food(&snake);

    let mut game_over = false;
    let mut score = 0;

    // Main game loop
    while let Some(event) = window.next() {

        // Keyboard controls
        if let Some(Button::Keyboard(key)) = event.press_args() {

            match key {

                Key::Up | Key::W => {
                    snake.change_direction(Direction::Up);
                }

                Key::Down | Key::S => {
                    snake.change_direction(Direction::Down);
                }

                Key::Left | Key::A => {
                    snake.change_direction(Direction::Left);
                }

                Key::Right | Key::D => {
                    snake.change_direction(Direction::Right);
                }

                Key::R if game_over => {
                    snake = Snake::new(5, 10);
                    food = random_food(&snake);
                    game_over = false;
                    score = 0;

                    window.set_title("Snake Game".to_string());
                }

                _ => {}
            }
        }

        // Update game
        if event.update_args().is_some() && !game_over {

            if let Some(food_position) = food {

                let old_length = snake.len();

                // Move snake
                if !snake.update(WIDTH, HEIGHT, food_position) {

                    game_over = true;

                    window.set_title(
                        format!("Game Over! Score: {} | Press R to Restart", score)
                    );

                } else if snake.len() > old_length {

                    // Snake ate food
                    score += 1;

                    println!("Score: {}", score);

                    food = random_food(&snake);

                    if food.is_none() {
                        game_over = true;
                        window.set_title(
                            format!("You Win! Score: {} | Press R to Restart", score)
                        );
                    } else {
                        window.set_title(
                            format!("Snake Game | Score: {}", score)
                        );
                    }
                }
            }
        }

        // Draw game
        window.draw_2d(&event, |con, g, _| {

            // Background
            if game_over {
                clear([0.25, 0.05, 0.05, 1.0], g);
            } else {
                clear([0.08, 0.08, 0.08, 1.0], g);
            }

            // Draw snake
            snake.draw(&con, g);

            // Draw food
            if let Some((x, y)) = food {
                draw_block(FOOD_COLOR, x, y, &con, g);
            }
        });
    }
}