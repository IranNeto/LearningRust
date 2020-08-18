fn main() {

    //This is a literal and it can be appended to another literal
    //since it is stored in the stack which must have the same size
    let mut x = "qwer";
    //x.push_str(" asdf");

    //String is a type that can be any size because of that is stored in the heap
    let s = String::from("qwer");
    s.push_str(" asdf");
    println!("{}", s);

    //Rust has a concept of moving. At the second line s2 receives the pointer
    //to s1 content. But in order to avoid double free memory bug when
    //this two variable get out of the scope (trying to deallocate the memory
    //space twice) s1 is invalidated
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world", s1);

    //To deeply copy something we use clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    //It's possible just bind integer values to other variable
    //because integer has a know size and is stored always on stack
    //So having a .clone() or not makes no difference
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
