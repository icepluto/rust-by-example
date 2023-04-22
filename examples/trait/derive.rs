

#[derive(PartialEq,PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches{
    fn to_centimeters(&self)->Centimeters{
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}
fn main(){
    let foot = Inches(50);

    let meter = Centimeters(100.00);
    let cmp = if foot.to_centimeters()>meter{
        "bigger"
    }else{
        "smaller"
    };
    println!("{}",cmp);
}