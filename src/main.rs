// We import the std::io library to read user input
use std::io;
// And then we import the package we just installed to generate random numbers
// Under the hood, this library does a lot of low-level operations to generate true random numbers
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // We generate an random integer with the i32 type
    let secret_number: i32 = rand::thread_rng().gen_range(1..101);
    // We could also write this as
        // let secret number: i32 = rand::thread_rng().gen_range(1..=100)
        // 1..=100 includes the maximum value of the range as well, while 1..100 doesn't
    
    // We print our secret_number (this is temporary and we do this only to check the number we generate)
        // We pass only one variable to println!, so here {} means "get the first variable passed in and add it to the string"
        // If we had multiple curly brackets and multiple variables passed into println, 
            //it'd looked through them all and add them in order
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
}
