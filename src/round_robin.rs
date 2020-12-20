use crate::game::Game;
use crate::user::User;

pub struct RoundRobin {
  head: User,
  rest: Vec<User>,
  round: u8,
}

impl RoundRobin {
  pub fn new(users: Vec<User>) -> RoundRobin {
    let mut local_users = users.clone();
    let head: User = local_users.pop().unwrap();

    RoundRobin {
      head,
      rest: local_users,
      round: 1,
    }
  }

  pub fn generate_pairings(&mut self) -> Vec<Vec<Game>> {
    let mut pairings: Vec<Vec<Game>> = vec![];
    for _ in (1..(self.rest.len() + 1)).enumerate() {
      pairings.push(self.generate_round());
      self.rotate();
    }

    pairings
  }

  fn generate_round(&self) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];

    if self.even_round() {
      games.push(Game::new(
        self.head.clone(),
        self.rest.get(0).unwrap().clone(),
      ));
    } else {
      games.push(Game::new(
        self.rest.get(0).unwrap().clone(),
        self.head.clone(),
      ));
    }

    for (i, _) in (0..(self.rest.len() / 2)).enumerate() {
      games.push(Game::new(
        self.rest.get(i + 1).unwrap().clone(),
        self.rest.get(self.rest.len() - 1 - i).unwrap().clone(),
      ))
    }

    games
  }

  fn rotate(&mut self) {
    let len = self.rest.len();
    self.rest.rotate_left((len + 1) / 2);
    self.round += 1;
  }

  fn even_round(&self) -> bool {
    self.round & 1 == 0
  }
}
