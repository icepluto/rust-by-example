#[derive(Debug)]
struct Structure(i32);


#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a>{
    name:&'a str,
    age:u8,
}
fn main(){
    let s = Structure(12);
    let d = Deep(Structure(1));
    let person = Person{
        name:"lanshi",
        age:24,
    };
    println!("my girl {} is {} years old.",person.name,person.age);
    println!("{:#?}",person);
    println!("{:?}",12);
    println!("{1:?}:{0:?}:{t:?}",0,1,t = 2);
    println!("{:?}",s);
    println!("{:?}",d);
}