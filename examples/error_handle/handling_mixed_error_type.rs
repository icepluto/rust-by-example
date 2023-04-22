use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|f| f.parse::<i32>().map(|x| x * 2));
    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let v = vec!["1", "2", "3"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    let f = double_first(v);
    let empty_f = double_first(empty);
    let strings = double_first(strings);
    println!("{:?}", f);
    println!("{:?}", empty_f);
    println!("{:?}", strings);
}
