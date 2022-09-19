use std::io;

fn main() {
    println!("Guess the number!");
    println!("plos Input your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");
    println!("you guessed this mf {guess}"); 
}
