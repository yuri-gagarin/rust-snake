use std::collections::LinkedList;

use piston_window::{ Context, G2d };
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [ 0.00, 0.00, 0.00, 1.0 ];

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32 
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>
}

impl Snake {
    pub fn new_snake(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        // create a new Snake linked list //
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        return Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }
    pub fn draw(&self, context: &Context, g: &mut G2d) {
        for block in &self.body {
          draw_block(SNAKE_COLOR, block.x, block.y, context, g)
        }
    }
    pub fn head_position(&self) -> (i32, i32) {
        let head_block: &Block = self.body.front().unwrap();
        return (head_block.x, head_block.y);
    }
    pub fn move_forward(&mut self, direction: Option<Direction>) {
        match direction {
            Some(d) => self.direction = d,
            None => ()
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block: Block = match self.direction {
            Direction::Up => Block { x: last_x, y: last_y - 1 },
            Direction::Down => Block { x: last_x, y: last_y + 1 },
            Direction::Right => Block { x: last_x + 1, y: last_y },
            Direction::Left => Block { x: last_x - 1, y: last_y }
        };

        // update the front of the snake //
        self.body.push_front(new_block);
        // update the tail of the snake //
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }
    pub fn head_direction(&self) -> Direction {
        return self.direction;
    }
}



