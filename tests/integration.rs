use fake::faker::name::raw::*;
use fake::locales::*;

use round_robin_pairing;

#[test]
fn should_generate_pairings_with_three_players() {
  let users = fake::vec![String as Name(EN); 3];

  let (_users, raw_pairings) = round_robin_pairing::generate(users.clone()).unwrap();

  let pairings = round_robin_pairing::format_pairings(raw_pairings);

  assert_eq!(
    pairings,
    vec![
      vec![(1, 0), (2, 3)],
      vec![(0, 3), (1, 2)],
      vec![(2, 0), (3, 1)]
    ]
  )
}

#[test]
fn should_generate_pairings_with_three_players_and_format_by_name() {
  let users = fake::vec![String as Name(EN); 3];

  let (users, raw_pairings) = round_robin_pairing::generate(users.clone()).unwrap();

  let pairings = round_robin_pairing::format_pairings_by_username(raw_pairings);

  assert_eq!(
    pairings,
    vec![
      vec![
        (
          users.iter().find(|user| { user.number == 1 }).unwrap().name.clone(),
          users.iter().find(|user| { user.number == 0 }).unwrap().name.clone()
        ),
        (
          users.iter().find(|user| { user.number == 2 }).unwrap().name.clone(),
          users.iter().find(|user| { user.number == 3 }).unwrap().name.clone()
        )
      ],
      vec![
        (
          users.iter().find(|user| { user.number == 0 }).unwrap().name.clone(),
          users.iter().find(|user| { user.number == 3 }).unwrap().name.clone()
        ),
        (
          users.iter().find(|user| { user.number == 1 }).unwrap().name.clone(),
          users.iter().find(|user| { user.number == 2 }).unwrap().name.clone()
        )
      ],
      vec![
        (
          users.iter().find(|user| { user.number == 2 }).unwrap().name.clone(),
          users.iter().find(|user| { user.number == 0 }).unwrap().name.clone()
        ),
        (
          users.iter().find(|user| { user.number == 3 }).unwrap().name.clone(),
          users.iter().find(|user| { user.number == 1 }).unwrap().name.clone()
        )
      ]
    ]
  )
}

#[test]
fn should_generate_pairings_with_four_players() {
  let users = fake::vec![String as Name(EN); 4];

  let (_users, raw_pairings) = round_robin_pairing::generate(users.clone()).unwrap();

  let pairings = round_robin_pairing::format_pairings(raw_pairings);

  assert_eq!(
    pairings,
    vec![
      vec![(1, 4), (2, 3)],
      vec![(4, 3), (1, 2)],
      vec![(2, 4), (3, 1)]
    ]
  )
}

#[test]
fn should_generate_pairings_with_five_players() {
  let users = fake::vec![String as Name(EN); 5];

  let (_users, raw_pairings) = round_robin_pairing::generate(users.clone()).unwrap();

  let pairings = round_robin_pairing::format_pairings(raw_pairings);

  assert_eq!(
    pairings,
    vec![
      vec![(1, 0), (2, 5), (3, 4)],
      vec![(0, 4), (5, 3), (1, 2)],
      vec![(2, 0), (3, 1), (4, 5)],
      vec![(0, 5), (1, 4), (2, 3)],
      vec![(3, 0), (4, 2), (5, 1)]
    ]
  )
}

#[test]
fn should_generate_pairings_with_six_players() {
  let users = fake::vec![String as Name(EN); 6];

  let (_users, raw_pairings) = round_robin_pairing::generate(users.clone()).unwrap();

  let pairings = round_robin_pairing::format_pairings(raw_pairings);

  assert_eq!(
    pairings,
    vec![
      vec![(1, 6), (2, 5), (3, 4)],
      vec![(6, 4), (5, 3), (1, 2)],
      vec![(2, 6), (3, 1), (4, 5)],
      vec![(6, 5), (1, 4), (2, 3)],
      vec![(3, 6), (4, 2), (5, 1)]
    ]
  )
}

#[test]
fn should_generate_pairings_with_zero_players() {
  let users = vec![];

  let result = round_robin_pairing::generate(users.clone());

  assert_eq!(result.is_none(), true);
}
