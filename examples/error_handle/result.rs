use std::num::ParseIntError;

fn multiply(first_number:&str,second_number:&str)->Result<i32,ParseIntError>{
    match first_number.parse::<i32>(){
        Ok(first_number)=>match second_number.parse::<i32>(){
            Ok(second_number)=>{
                Ok(first_number*second_number)
            },
            Err(e)=>Err(e),
        },
        Err(e)=>Err(e),
    }
}
fn print(number:Result<i32,ParseIntError>){
    match number{
        Ok(num)=>println!("{}",num),
        Err(e)=>println!("{}",e),
    }
}
fn main(){
    let (f,s) = ("1","2");
    
    let res = multiply(f,s);
    let res1 = multiply("a", "1");
    print(res);
    print(res1);
}