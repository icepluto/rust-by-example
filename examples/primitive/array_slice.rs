fn analyzer_slice(s:&[i32]){
    println!("the first of slice element is {}",s[0]);
    println!("slice length is {}",s.len());
}
fn main(){
    let slice:[i32;100] = [11;100];
    analyzer_slice(&slice);
    let xs:[i32;6] = [1,2,3,4,5,6];
    println!("first of vec is {}",xs[0]);
    println!("second of vec is {}",xs[1]);

    println!("xs的分配的大小是:{}",std::mem::size_of_val(&xs));
}