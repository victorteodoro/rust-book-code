fn inc(x: i32) -> i32 {
    x + 1
}

fn dec(x: i32) -> i32 {
    x - 1
}

fn main() {
    let x = 2;
    
    let y = if x < 5 { inc(x) } else { dec(x) };

    println!("The value of y is: {}", y);

    let x = Some(5);

    let y = if let Some(y) = x { y } else { 3 };

    println!("The value of y is {}.", y);
}
