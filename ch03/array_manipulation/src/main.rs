use std::any::type_name;
use std::mem;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements\n", slice.len());
}

fn main() {
    let xs = [1, 2, 3, 4, 5];
    let ys = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);
    println!("number of elements in array: {}", xs.len());
    println!("array occupies {} bytes\n", mem::size_of_val(&xs));
    println!("type of xs: {}", type_of(xs));
    println!("type of ys: {}\n", type_of(ys));

    println!("borrow the whole array as a slice");
    analyze_slice(&ys);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[0..50]);

    // Out of bound indexing causes compile error
    //println!("{}", xs[5]);
}
