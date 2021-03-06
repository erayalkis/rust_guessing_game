// We import the std::io library to read user input
use std::io;
// And then we import the package we just installed to generate random numbers
// Under the hood, this library does a lot of low-level operations to generate true random numbers
use rand::Rng;
// We import the Ordering enumerable here to match our input and check if it's lesser, 
    // equal or greater than our secret number.
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // We generate an random integer with the i32 type
    // The gen_range method takes a range expression and generates a number within that range
    // Range expressions take the form of "start..end", end being exclusive. If you want an inclusive range,
        // you can use the "start..=end" syntax instead.
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

    // By default, all variables in Rust are immutable, meaning that you can't change their value
     // However, if we wish to do this, we can add the "mut" keyword while creating the variable.
    // Here, we create a mutable empty String named "guess", this will hold our guessed number.
    let mut guess = String::new();

    // Here, we call the stdin method, and call read_line on it.
    // read_line takes an empty string to store the input value in it. 
    // Instad of passing the variable directly, we say &mut guess instead. But why?
    // Here & means that we're *borrowing* the value. When we're passing variables around,
        // Rust forces us to borrow them so that we don't create a new place in memory for them each time.
    // But then, what does "&mut" mean? In Rust, all borrows are also immutable. So if you want to borrow a mutable variable,
        // and then change the borrowed variables value, you have to use &mut.
    // So with all of this knowledge, we borrow our guess String, and pass it into read_line.
        // read_line takes our variable, and adds the input into it(mutates the variable).
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!"); // This line here will basically raise an error when anything unexpected happens.
        // This works with the Result enumerable, but that's for later.
    
    // Uh oh, this throws an error! But why?
    // Oh, the cmp method expects a String, but we're passing in an i32!
    // This happens because our guess variable is a String, and to match it against our secret number, 
         // the number also needs to be a String!
    // We can fix this by making our guess variable into an i32!
    // Now, we need to go back and parse our guess variable! See 01.rs
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }

}
