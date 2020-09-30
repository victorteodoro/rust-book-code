fn main() {
    // With tuples
    let tup = (1, 2, 3);
    let (_, _, third) = tup;
    println!("The third element is {}", third);

    // With Options
    let x = Some(5);
    match x {
        Some(y) => println!("y is {}", y),
        None => println!("Something went wrong with y"),
    }
}
