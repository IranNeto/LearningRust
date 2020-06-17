fn main() {
    /*
     mut assign the mutability of a variable 
     */
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /*
    Rust stablish uppercase and underscore between words and in numbers to improve readability
    const are always expressions
    */
    const MAX_POINTS: u32 = 100_000;

    /*
    shadowing a variable allows to reuse a its name and use its value in some operation to reassign.
    
    Its different from a mutable variable because it's not the process of create a new variable, or allocate 
    another memory space but reassign its values and even reassign its type.

    As occurs with z variable 'let mut' instruction creates a new memory allocation instead of reusing.
    */
    let y = 5;
    let y_addr = &y;
    
    println!("Y_address {:p}", y_addr);

    let y = y + 1;

    println!("Y_address after shadowing once {:p}", y_addr);

    let y = y * 2;

    println!("Y_address after shadowing twice {:p}", y_addr);
    println!("The value of y is: {}", y);

    let mut z = 4;
    let z_addr = &z;

    println!("Z_address mutable addr {:p}", &z as *const i32);
    println!("z value: {}", z);

    let mut z = 40;

    let z_addr = &z;
    println!("Z_address after reassign {:p}", &z);

    println!("z value: {}", z);
}