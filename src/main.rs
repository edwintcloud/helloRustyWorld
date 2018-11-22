extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
  println!("Hello rusty world!");

  // generate a random number
  let rand_num = rand::thread_rng().gen_range(1, 101);

  // loop until correct number guessed
  loop {
    println!("Please guess a number between 1 and 100: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("failed to read line womp");

    // convert guess to number or ask again
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    // let user know how close they are
    match guess.cmp(&rand_num) {
      Ordering::Less => println!("A bit higher.."),
      Ordering::Greater => println!("Too much!!"),
      Ordering::Equal => {
        println!("You guessed it!! {}", guess);
        break;
      }
    }
  }
}