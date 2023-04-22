fn give_my_princess(gift:&str){
    if gift == "snake"{ panic!("aaaaaaaaaaa!!!!")}
    println!("i love {}s",gift)
}

fn main(){
    give_my_princess("snake");
    give_my_princess("apple");
}