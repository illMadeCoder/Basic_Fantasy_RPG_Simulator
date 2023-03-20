use std::str::FromStr;

use crate::point::Point;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub const DIRECTIONS: &[Direction] = &[
    Direction::Left,
    Direction::Right,
    Direction::Up,
    Direction::Down,
];

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
        }
    }
}

impl From<&Direction> for Point {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
        }
    }
}

#[derive(Debug)]
pub struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "left" => Ok(Direction::Left),
            "right" => Ok(Direction::Right),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(ParseDirectionError),
        }
    }
}
