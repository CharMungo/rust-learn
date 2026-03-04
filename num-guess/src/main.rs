use std::io;
use rand::prelude::*;

fn main() {
    println!("Input upper limit");
    let mut rng = rand::rng();
    let mut limitStr = String::new();
    io::stdin().read_line(&mut limitStr).expect("Failed");
    let limit: usize = limitStr.trim().parse().expect("Failed to parse");
    println!("Limit: {}", limitStr);
    let num = rng.random_range(1..limit);
    let mut guessStr = String::new();
    println!("Input guess");
    io::stdin().read_line(&mut guessStr).expect("Failed");
    let guess: usize = guessStr.trim().parse().expect("Failed to parse guess");
    println!("Guess: {}", guessStr);
    println!("Answer: {}", num);
    if num == guess {
        println!("Correct");
    } else {
        println!("Wrong");
    }
}
