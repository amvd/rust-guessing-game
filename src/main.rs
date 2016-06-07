extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to my 'Guess The Number' game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
      println!("\nPlease input your guess.\n");

      let mut guess = String::new();

      io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          println!("Not a number.\n");
          continue;
        },
      };

      match guess.cmp(&secret_number) {
        Ordering::Less    => println!("\n{} is too small!", guess),
        Ordering::Greater => println!("\n{} is too big!", guess),
        Ordering::Equal   => {
          println!("\n{} is the correct number!\n\nYou win!", guess);
          break;
        },
      }
    }
}
