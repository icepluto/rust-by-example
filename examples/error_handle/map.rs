#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);

#[derive(Debug)]
struct Chopped(Food);

#[derive(Debug)]
struct Cooked(Food);

fn peel(food:Option<Food>)->Option<Peeled>{
    match food{
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}
fn chop(peeled:Option<Peeled>)->Option<Chopped>{
    match peeled{
        Some(Peeled(food))=>Some(Chopped(food)),
        None=>None,
    }
}
fn cook(chopped:Option<Chopped>)->Option<Cooked>{
    chopped.map(|Chopped(food)|Cooked(food))
}

fn process(food:Option<Food>)->Option<Cooked>{
    food.map(|f|Peeled(f))
        .map(|Peeled(f)|Chopped(f))
        .map(|Chopped(f)|Cooked(f))
}
fn eat(food:Option<Cooked>){
    match food {
        Some(f)=> println!("l like {:?}",f),
        None => println!("no,it wasn't edible")
    }
}

fn main(){
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cook_apple = cook(chop(peel(apple)));
    let cook_carrot = cook(chop(peel(carrot)));

    let cook_potato = process(potato);

    eat(cook_apple);
    eat(cook_carrot);
    eat(cook_potato);
}