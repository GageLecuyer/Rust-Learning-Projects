use piston_window::graphics::{Context, Graphics};
use crate::draw::draw_block;

const SNAKE_COLOR: [f32; 4] = [0.0, 0.8, 0.0, 1.0];

// Represents one part of the snake
#[derive(Clone, Copy, PartialEq)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

// Directions the snake can move
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Our snake
pub struct Snake {
    body: Vec<Block>,
    direction: Direction,
    next_direction: Direction,
}

impl Snake {

    // Creates a new snake
    pub fn new(x: i32, y: i32) -> Snake {
        Snake {
            body: vec![
                Block { x, y },
                Block { x: x - 1, y },
                Block { x: x - 2, y },
            ],
            direction: Direction::Right,
            next_direction: Direction::Right,
        }
    }

    // Draws every part of the snake
    pub fn draw<G: Graphics>(&self, con: &Context, g: &mut G) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }
    // Changes the snake's direction
    pub fn change_direction(&mut self, new_direction: Direction) {

        // Prevent turning directly backwards
        let opposite = matches!(
            (self.direction, new_direction),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        );

        if !opposite {
            self.next_direction = new_direction;
        }
    }

    // Moves the snake forward
    // Returns false if the snake crashes
    pub fn update(
        &mut self,
        width: i32,
        height: i32,
        food: (i32, i32),
    ) -> bool {

        let head = self.body[0];

        let mut new_head = head;

        match self.next_direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        // Check wall collisions
        if new_head.x < 0
            || new_head.x >= width
            || new_head.y < 0
            || new_head.y >= height
        {
            return false;
        }

        // Check if snake ate food
        let ate_food = new_head.x == food.0 && new_head.y == food.1;

        // The tail moves away unless food is eaten
        let check_length = if ate_food {
            self.body.len()
        } else {
            self.body.len() - 1
        };

        // Check if snake hits itself
        for block in self.body.iter().take(check_length) {
            if *block == new_head {
                return false;
            }
        }

        // Add new head
        self.body.insert(0, new_head);

        // Remove the tail unless food was eaten
        if !ate_food {
            self.body.pop();
        }

        self.direction = self.next_direction;

        true
    }

    // Check whether a position contains the snake
    pub fn contains(&self, x: i32, y: i32) -> bool {
        for block in &self.body {
            if block.x == x && block.y == y {
                return true;
            }
        }

        false
    }

    // Get the snake's length
    pub fn len(&self) -> usize {
        self.body.len()
    }
}