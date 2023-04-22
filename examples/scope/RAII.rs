struct ToDrop;
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("todrop is droped");
    }
}
fn main(){
    let _x = ToDrop;
    println!("made a todrop");
}