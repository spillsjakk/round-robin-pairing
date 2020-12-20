#[derive(Clone, Debug)]
pub struct User {
  pub number: usize,
  pub name: String,
}

impl User {
  pub fn new(number: usize, name: &str) -> User {
    User {
      number,
      name: name.to_string(),
    }
  }

  pub fn from_names(user_names: Vec<String>) -> Vec<User> {
    let mut users: Vec<User> = vec![];
    for (i, user) in user_names.iter().enumerate() {
      users.push(User::new(i + 1, user))
    }
    users
  }
}
