use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn main() {
    let tup = (500, 6.4, 1);
    let (_, x, _) = tup;
    println!("The value of x is: {}", x);
    println!("The first item is: {}", tup.0);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    println!("type of tuple of tuples: {}", type_of(tuple_of_tuples));

    let pair = (100, false);
    println!("pair reversed is: {:?}", reverse(pair));
    println!("type of pair: {}", type_of(pair));
}
