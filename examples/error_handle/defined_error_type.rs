use std::{error::Error, fmt, result};

type Res<T> = result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "没找到第一个元素啊")
    }
}
impl Error for DoubleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        //泛型错误，没有记录原因
        None
    }
}

fn double_first(v: Vec<&str>) -> Res<i32> {
    //first 返回第一个元素，切片，如果是empty则返回none。返回值为option
    v.first()
        //ok_or 转换option<T>为result<T,E>,some()=>OK(),None=>Err
        .ok_or(DoubleError)
        //如果前面是Err()那返回err的值,不然就执行闭包
        .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i))
}

fn print(res:Res<i32>){
    match res{
        Ok(f)=>println!("{}",f),
        Err(e)=>println!("你这里有错误哈，错误是： {}",e)
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
