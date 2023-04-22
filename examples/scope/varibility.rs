fn main() {
    let immutable = Box::new(5i32);
    // *immutable = 6;
    println!("{}", immutable);

    let mut mutable = immutable;
    println!("{}", mutable);
    *mutable = 6i32;

    println!("{}", mutable);
}
