use std::num::ParseIntError;

type Alias<T> = Result<T,ParseIntError>;

fn multiply(first_number: &str, second_number: &str) -> Alias<i32> {
    first_number
        .parse::<i32>()
        .and_then(|f| second_number.parse::<i32>().map(|s| f * s))
}
fn print(number: Alias<i32>) {
    match number {
        Ok(num) => println!("{}", num),
        Err(e) => println!("{}", e),
    }
}
fn main() {
    let (f, s) = ("5", "2");

    let res = multiply(f, s);
    let res1 = multiply("a", "1");
    print(res);
    print(res1);
}
