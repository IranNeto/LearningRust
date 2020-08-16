enum Direction {
    Up, Down, Stay
}


fn main() {
    let myDirection:Direction = Direction::Stay;

    match myDirection {
        Direction::Up => println!("You're going Up"),
        Direction::Down => println!("You're going down"),
        _ => println!("Something goes wrong")
    }
}
