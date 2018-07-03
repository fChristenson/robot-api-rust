use std::iter::*;

pub enum Direction {
  North,
  East,
  South,
  West,
}

impl Direction {
  pub fn value(&self) -> &str {
    match *self {
      Direction::North => "north",
      Direction::East => "east",
      Direction::South => "south",
      Direction::West => "west",
    }
  }
}

impl Clone for Direction {
  fn clone(&self) -> Direction {
    match &self {
      Direction::North => Direction::North,
      Direction::East => Direction::East,
      Direction::South => Direction::South,
      Direction::West => Direction::West,
    }
  }
}

impl Iterator for Direction {
  type Item = Direction;

  fn next(&mut self) -> Option<Direction> {
    match self {
      Direction::North => Some(Direction::East),
      Direction::East => Some(Direction::South),
      Direction::South => Some(Direction::West),
      Direction::West => Some(Direction::North),
    }
  }
}

impl DoubleEndedIterator for Direction {
  fn next_back(&mut self) -> Option<Direction> {
    match self {
      Direction::North => Some(Direction::West),
      Direction::East => Some(Direction::North),
      Direction::South => Some(Direction::East),
      Direction::West => Some(Direction::South),
    }
  }
}
