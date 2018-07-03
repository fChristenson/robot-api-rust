use lib::direction::Direction;

pub struct Robot {
  pub facing: Direction,
}

impl Robot {
  pub fn turn_right(&mut self) -> &mut Robot {
    self.facing = self.facing.next().unwrap();
    self
  }

  pub fn turn_left(&mut self) -> &mut Robot {
    self.facing = self.facing.clone().rev().next().unwrap();
    self
  }
}
