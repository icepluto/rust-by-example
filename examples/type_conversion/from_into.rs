#[derive(Debug)]
struct Number {
    value:i32,
}

impl From<i32> for Number{
    fn from(value: i32) -> Self {
        Number { value}
    }
}
fn main(){
    let number = Number::from(30);
    println!("{:?}",number);

    let num:Number = 5.into();
    println!("{:?}",num);
}