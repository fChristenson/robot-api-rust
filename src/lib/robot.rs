use lib::direction::Direction;
use lib::point::Point;

#[derive(Serialize, Deserialize)]
pub struct Robot {
  pub facing: Direction,
  pub position: Point,
}

impl Robot {
  pub fn move_forward(&mut self) -> &Robot {
    match self.facing {
      Direction::North => {
        let new_y = self.position.y.checked_add(1).unwrap_or(u32::max_value());
        self.position.y = new_y
      }
      Direction::East => {
        let new_x = self.position.x.checked_add(1).unwrap_or(u32::max_value());
        self.position.x = new_x
      }
      Direction::South => self.position.y = self.position.y.checked_sub(1).unwrap_or(0),
      Direction::West => self.position.x = self.position.x.checked_sub(1).unwrap_or(0),
    };

    self
  }

  pub fn turn_right(&mut self) -> &Robot {
    self.facing = self.facing.next().unwrap();
    self
  }

  pub fn turn_left(&mut self) -> &Robot {
    self.facing = self.facing.clone().rev().next().unwrap();
    self
  }
}
