#![allow(unused)]
#[derive(Debug)]
enum Food{
    Steak,
    Sushi,
    CordonBleu,
}
#[derive(Debug)]
enum Day{
    Monday,
    Tuesday,
    Wedesday,
}
fn have_ingredients(food:Food)->Option<Food>{
    match food{
        Food::Sushi=>None,
        _=>Some(food),
    }
}
fn have_recipe(food:Food)->Option<Food>{
    match food{
        Food::CordonBleu=>None,
        _=>Some(food),
    }
}
fn cookable_v1(food:Food)->Option<Food>{
    match have_ingredients(food){
        None=>None,
        Some(food)=>match have_recipe(food){
            None=>None,
            Some(food)=>Some(food),
        }
    }
}
fn cookable_v2(food:Food)->Option<Food>{
    have_ingredients(food).and_then(have_recipe)
}

fn eat(food:Food,day:Day){
    match cookable_v2(food){
        Some(food)=>println!("on {:?} we get to eat {:?}",day,food),
        None=>println!("oh,we won't get to eat on {:?}",day),
    }
}
fn eat1(food:Food,day:Day){
    match cookable_v1(food){
        Some(food)=>println!("on {:?} we get to eat {:?}",day,food),
        None=>println!("oh,we won't get to eat on {:?}",day),
    }
}
fn main(){
    let (cordon_bleu,steak,sushi) = (Food::CordonBleu,Food::Steak,Food::Sushi);
    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Wedesday);
    eat(sushi, Day::Tuesday);
    // eat1(cordon_bleu, Day::Monday);
    // eat1(steak, Day::Wedesday);
    // eat1(sushi, Day::Tuesday);
}