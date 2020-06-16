use std::io; // io library
use rand::Rng;
use std::cmp::Ordering;

fn main() { // entry point in every program

    loop {
        println!();
        println!("Guess the number");
        println!("Please input your guess.");
    
        /*
        Rng is a trait and define methods that random number
        generators implement.
        
        ::thread_rng() returns a random number generator. This is a
        particular one once is local to the current thread execution
        and seeded by OS.
    
        gen_reange() generate a random number into the range [1,10)
        */
        let secret_number = rand::thread_rng().gen_range(1, 10);
    
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
    
        /*
        let guess means that Rust allows shadow variables
        .trim() eliminates whitespace
        .parse() parse string in some numerical type, because of the range of types parse can manipulate
        is necessary to explicitly give the type
    
        add a match operator to process Result coming from .parse() in order to ignore when
        user's input is invalid.
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };
        
        println!("You guessed: {}", guess);
    
        /*
        std::cmp::Ordering is a enum like Result
        .cmp() compares 2 values and returns an Ordering variant
        
        match is a expression made up of arms which is a pattern.
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}