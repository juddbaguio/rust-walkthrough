use std::io::{stdin};

use rand::{random};

fn main() {
    println!("Guess the freaking number!");
    println!("Input your guess number! woowwww");
    
    let mut guess_input = String::new();

    stdin().read_line(&mut guess_input)
    .expect("failed to read line");

    println!("you guessed {guess_input}");
    let new_var = guess_input.lines().collect::<String>();

    let secret_multiplier = random::<i64>();
    println!("{secret_multiplier}");
    let mut random_new_input = new_var.parse::<i64>().unwrap();
    random_new_input = random_new_input*secret_multiplier;
    println!("the doubled num is {random_new_input}");

}
