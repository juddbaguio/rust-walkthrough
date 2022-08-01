use std::io::stdin;

fn main() {
    println!("Guess the freaking number!");
    println!("Input your guess number! woowwww");
    
    let mut guess_input = String::new();

    stdin().read_line(&mut guess_input)
    .expect("failed to read line");


    println!("you guessed {guess_input}");
}
