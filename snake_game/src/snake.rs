use crate::draw::draw_block;
use piston_window::graphics::{Context, Graphics};

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
            body: vec![Block { x, y }, Block { x: x - 1, y }, Block { x: x - 2, y }],
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
    pub fn update(&mut self, width: i32, height: i32, food: (i32, i32)) -> bool {
        let head = self.body[0];

        let mut new_head = head;

        match self.next_direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        // Check wall collisions
        if new_head.x < 0 || new_head.x >= width || new_head.y < 0 || new_head.y >= height {
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

#[cfg(test)]
mod tests {
    use super::*;

    // Food placed somewhere the snake will not reach during a test
    const FAR_FOOD: (i32, i32) = (19, 19);

    #[test]
    fn new_snake_has_three_blocks() {
        let snake = Snake::new(5, 10);
        assert_eq!(snake.len(), 3);
        assert!(snake.contains(5, 10));
        assert!(snake.contains(4, 10));
        assert!(snake.contains(3, 10));
        assert!(!snake.contains(6, 10));
    }

    #[test]
    fn moves_right_by_default() {
        let mut snake = Snake::new(5, 10);
        assert!(snake.update(20, 20, FAR_FOOD));
        assert!(snake.contains(6, 10));
        assert!(!snake.contains(3, 10));
        assert_eq!(snake.len(), 3);
    }

    #[test]
    fn grows_when_eating_food() {
        let mut snake = Snake::new(5, 10);
        assert!(snake.update(20, 20, (6, 10)));
        assert_eq!(snake.len(), 4);
        assert!(snake.contains(3, 10));
    }

    #[test]
    fn crashes_into_wall() {
        let mut snake = Snake::new(19, 10);
        assert!(!snake.update(20, 20, FAR_FOOD));
    }

    #[test]
    fn cannot_reverse_direction() {
        let mut snake = Snake::new(5, 10);
        snake.change_direction(Direction::Left);
        assert!(snake.update(20, 20, FAR_FOOD));
        assert!(snake.contains(6, 10));
    }

    #[test]
    fn turns_up() {
        let mut snake = Snake::new(5, 10);
        snake.change_direction(Direction::Up);
        assert!(snake.update(20, 20, FAR_FOOD));
        assert!(snake.contains(5, 9));
    }

    #[test]
    fn crashes_into_itself() {
        // Grow to length 5, then turn in a tight loop back into the body
        let mut snake = Snake::new(5, 10);
        assert!(snake.update(20, 20, (6, 10)));
        assert!(snake.update(20, 20, (7, 10)));
        snake.change_direction(Direction::Down);
        assert!(snake.update(20, 20, FAR_FOOD));
        snake.change_direction(Direction::Left);
        assert!(snake.update(20, 20, FAR_FOOD));
        snake.change_direction(Direction::Up);
        assert!(!snake.update(20, 20, FAR_FOOD));
    }
}
