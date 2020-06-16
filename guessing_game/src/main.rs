use std::io; // io library
use rand::Rng;

fn main() { // entry point in every program
    println!("Guess the number");
    println!("Please input your guess.");

    /*
    Rng is a trait and define methods that random number
    generators implement.
    
    ::thread_rng() returns a random number generator. This is a
    particular one once is local to the current thread execution
    and seeded by OS.

    gen_reange() generate a random number into the range [1,101)
    */
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    println!("The secret number is: {}", secret_number);

    /*
    'let' is used to create a variable
    'mut' indicates a mutable variable. Every variable is immutable by default

    :: indicates a associated function (static method)
    Strin::new() return a new instace of a String
    */
    let mut guess = String::new();

    /*
    io::stdin() returns a new instace of a Stdin (handle to terminal's input)
    .read_line(&mut guess) hadle the input and put into guess
    & indicate that this method is a reference
    
    .read_line() returns an io::Result encapsulating an error
    */
    io::stdin()
    .read_line(&mut guess)
    .expect("Fauled to read line");

    println!("You guessed: {}", guess);
}