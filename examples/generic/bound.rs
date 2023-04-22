use std::fmt::Debug;

trait HasArea{
    fn area(&self)->f64;
}

impl HasArea for Rectangle{
    fn area(&self)->f64 {
        &self.heigth*&self.lenght
    }
}

#[derive(Debug)]
struct Rectangle{
    lenght:f64,
    heigth:f64
}

#[allow(dead_code)]
struct Triangle{
    lenght:f64,
    height:f64,
}

fn print_debug<T:Debug>(t:&T){
    println!("{:?}",t);
}

fn area<T:HasArea>(t:&T)->f64{t.area()}

fn main(){
    let reactangle = Rectangle{lenght:10.0,heigth:12.0};
    let _triangle = Triangle{lenght:12.0,height:20.0};

    print_debug(&reactangle);
    println!("{:?}",area(&reactangle));
}