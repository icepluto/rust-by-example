

fn print_refs<'a,'b>(x:&'a i32,y:&'b i32){
    println!("x is {} y is {}",x,y);
}

fn failed_demo<'a>(){
    let _x = 10;

    //这里y的生命周期是'a,比_x存活的长
    // let y:&'a i32 = &_x;
}
fn main(){
    let (x,y) = (4,9);
    //x y 存活的时间比print_refs的生命周期长
    print_refs(&x, &y);
    failed_demo();
}