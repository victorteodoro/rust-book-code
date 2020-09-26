fn inc(x: i32) -> i32 {
    x + 1
}

fn dec(x: i32) -> i32 {
    x - 1
}

fn main() {
    // Simple if expression
    // Types of the arms have to be the same
    let x = 2;
    let y = if x < 5 { inc(x) } else { dec(x) };

    println!("The value of y is: {}", y);

    // if expression with destrucutring
    let x = Some(5);
    let y = if let Some(y) = x { y } else { 3 };

    println!("The value of y is {}.", y);

    // Another one
    let x = None;
    let y = if let Some(y) = x { y } else { 3 };

    println!("The value of y is {}.", y);

    // Same thing with a match expression instead
    // Note that the types of the arms still need to be the same
    let x = Some(3);
    let a = match x {
        Some(1) => 1,
        Some(2) => 2,
        Some(y) => y,
        _ => -1,
    };

    println!("a is equal to {}", a);
}
