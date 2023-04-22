
fn give_commoner(gift:Option<&str>){
    match gift{
        Some("snake") => println!("oh!你怎么给了一条蛇啊，我要装起来！"),
        Some(inner)=>{
            println!("你给我{}花了多少钱呢？",inner);
        },
        None => println!("怎么给了我一个空的东西呢"),
    }
}

fn give_princess(gift:Option<&str>){
    let inside = gift.unwrap();
    if inside =="snake"{
        panic!("oh no, you scared me! ")
    }
    println!("i love {} and you",inside);
}

fn main(){
    let snake = Some("snake");
    let sheep = Some("sheep");
    let cow = Some("cow");
    let void= None;

    give_commoner(snake);
    give_commoner(sheep);
    give_commoner(cow);
    give_commoner(void);

    give_princess(sheep);
    give_princess(snake);

}