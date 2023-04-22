use std::fmt;

#[derive(Debug)]
struct MinMax(i32,i32);

impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({},{})",self.0,self.1)
    }
}

#[derive(Debug)]
struct Point2D{
    x:i32,
    y:i32,
}
impl fmt::Display for Point2D{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"x:{},y:{}",self.x,self.y)
    }
}

fn main(){
    let minmax = MinMax(1,11);
    println!("compare structures:");
    println!("debug:{:?}",minmax);
    println!("display:{}",minmax);

    let big_range = MinMax(-333,333);
    let small_range = MinMax(-3,3);

    println!("the big range is {big},the small is {small}",big = big_range,small = small_range);

    let point = Point2D{
        x:1,
        y:10,
    };

    println!("compare Point2D:");
    println!("debug:{:?}",point);
    println!("display:{}",point);
}
