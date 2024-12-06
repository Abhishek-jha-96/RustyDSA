use std::{cmp::Ordering, io};
use rand::Rng;
// if some thing is not in prelude, then it needs to be imported using "use".

fn random_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=100); // using a library method;
    return secret_number; // return the value and syntax of return.
}

fn taking_input() {
    println!("Guess the number!");

    let secret_number = random_number();

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
    // use "let" to create a variable, and "mut" is used to make it mutable.
    let mut guess = String::new();
    
    // io standard library is used for input/output
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("please type a number!"); // shadowing

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

fn main() {
    taking_input();

}
