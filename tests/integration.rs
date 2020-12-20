use fake::locales::*;
use fake::faker::name::raw::*;

use round_robin_pairing;

#[test]
fn should_generate_pairings_with_four_players() {
  let users = fake::vec![String as Name(EN); 4];

  let (_users, raw_pairings) = round_robin_pairing::generate(users.clone());

  let pairings = round_robin_pairing::format_pairings(raw_pairings);


  assert_eq!(pairings, vec![vec![(1, 4), (2, 3)], vec![(4, 3), (1, 2)], vec![(2, 4), (3, 1)]])
}

#[test]
fn should_generate_pairings_with_six_players() {
  let users = fake::vec![String as Name(EN); 6];
  println!("users: {:?}", users);

  let (_users, raw_pairings) = round_robin_pairing::generate(users.clone());

  let pairings = round_robin_pairing::format_pairings(raw_pairings);


  assert_eq!(pairings, vec![vec![(1, 6), (2, 5), (3, 4)], vec![(6, 4), (5, 3), (1, 2)], vec![(2, 6), (3, 1), (4, 5)], vec![(6, 5), (1,4), (2, 3)], vec![(3, 6), (4, 2), (5, 1)]])
}
