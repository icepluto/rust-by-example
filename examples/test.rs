fn main() {
    println!("Hello, world!");
    let a = Unprint{a:1};
    println!("{:?}",a);
}

#[derive(Debug)]
struct Unprint {
    a:i32,
}