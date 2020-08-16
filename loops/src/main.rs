fn main() {
    let mut n = 0;

    loop {
        n += 1;

        if n == 30000{
            break;
        }

        // Continue means skip the current loop and repeat from the top
        if true {
            continue;
        }

        println!("The value of n is {}", n);
    }

    println!("Finished");

    n = 0;

    while n <= 300 {
        println!("into the while {}", n);
        n += 1;
    }

    for i in 1..11{
        //for loop from 1 to 10
    }

    //create a range and go through it later
    let numbers = 11..50;
    for i in numbers {
        //for loop from 11 to 49
    }

    //Create a vector of strings (or slice of strings)
    let animals = vec!["Rabbit", "Dog", "Cat"];

    // animals.iter() call the iteration method of slices
    //Without .iter() it's not possible to access animals after for loop
    for animal in animals.iter() {
        println!("{}", animal);
    }

    //Go through the index and the string
    for (index, animal) in animals.iter().enumerate() {
        println!("{}, {}", index, animal);
    }
}
