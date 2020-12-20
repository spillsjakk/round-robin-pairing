use game::Game;
use rand::seq::SliceRandom;
use rand::thread_rng;
use round_robin::RoundRobin;
use user::User;

mod round_robin;
mod game;
mod user;

pub fn generate(user_names: Vec<String>) -> (Vec<User>, Vec<Vec<Game>>) {
  let mut rng = thread_rng();
  let mut local_user_names = user_names.clone();
  local_user_names.shuffle(&mut rng);

  let users = User::from_names(local_user_names);

  let mut round_robin = RoundRobin::new(users.clone());

  let pairings = round_robin.generate_pairings();

  (users, pairings)
}

pub fn format_pairings(raw_pairings: Vec<Vec<Game>>) -> Vec<Vec<(usize, usize)>> {
  let mut pairings: Vec<Vec<(usize, usize)>> = vec![];

  for (_, rounds) in raw_pairings.iter().enumerate() {
    let mut round: Vec<(usize, usize)> = vec![];
    for (_, game) in rounds.iter().enumerate() {
      round.push((game.white.number, game.black.number));
    }
    pairings.push(round);
  }

  pairings
}
