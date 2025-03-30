use std::io;

fn main() {
    println!("Guess a number between 1 and 100!");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let x = 5;
    let y = 10;
    println!("You guessed: {}", guess.trim());
    println!("x: {x}, x+y: {{x+y}}");
}
