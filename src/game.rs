use crate::User;

#[derive(Debug)]
pub struct Game {
  pub white: User,
  pub black: User,
}

impl Game {
  pub fn new(white: User, black: User) -> Game {
    Game { white, black }
  }
}
