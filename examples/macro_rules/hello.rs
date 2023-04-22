macro_rules! sayhello {
    () => {
        println!("hello macro_rules")
    };
}
fn main(){
    sayhello!()
}