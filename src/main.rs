use std::io;

fn main() {
  println!("Hello rusty world!");

  println!("Please input your favorite number: ");

  let mut fav_num = String::new();

  io::stdin().read_line(&mut fav_num)
      .expect("failed to read line womp");

  println!("No way... {} is my favorite number too!", fav_num);
}