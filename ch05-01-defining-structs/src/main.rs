struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn main() {
  // 構造体 Userを使ってuser1を作成する
  let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  // user1はmutableなので更新が可能
  println!("{:?}", user1.email);
  user1.email = String::from("anotheremail@example.com");
  println!("{:?}", user1.email);
  println!();

  // 構造体更新記法でuser2を作成する
  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
  };

  println!("{:?}",user2.username);
  println!("{:?}",user2.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}