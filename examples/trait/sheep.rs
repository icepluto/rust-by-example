
struct Sheep {
    name:&'static str,
    naked:bool,
}

trait Animal{
    fn new(name:&'static str)->Self;
    fn name(&self)->&'static str;
    fn noise(&self)->&'static str;
    fn talk(&self){
        println!("{} say {}",self.name(),self.noise());
    }
}
impl Sheep{
    fn is_naked(&self)->bool{
        self.naked
    }
    fn shear(&mut self){
        if self.is_naked(){
            println!("{} is already naked",self.name);
        }else{
            println!("{} is get haircut",self.name);
            self.naked = true;

        }
    }
}
impl Animal for Sheep{
    fn new(name:&'static str)->Self {
        Sheep { name, naked: false }
    }
    fn name(&self)->&'static str {
        self.name
    }
    fn noise(&self)->&'static str {
        if self.is_naked(){
            "baaaaah?? 毛怎么没了"
        }else{
            "baaaahh!!! 好热"
        }
    }
    fn talk(&self) {
        println!("{} is peaceful now and say {}",self.name,self.noise());
    }
}
fn main(){
    let mut dolly:Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.shear();
    dolly.talk();
}