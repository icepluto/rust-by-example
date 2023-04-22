struct A;
struct GenericVal<T>(T);

impl GenericVal<A>{}
impl GenericVal<i32>{}
impl <T> GenericVal<T>{}

struct Val{
    val:i32,
}
impl Val{
    fn value(&self)->&i32{
        &self.val
    }
}
struct GenVal<T>{
    gen_val:T
}
impl <T>GenVal<T>{
    fn value(&self)->&T{
        &self.gen_val
    }
}
fn main(){
    let x = Val{val:10};
    let y = GenVal{gen_val:10i32};
    println!("{}\r\n{}",x.value(),y.value());
}