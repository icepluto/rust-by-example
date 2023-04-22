fn main(){
    #[derive(Debug)]
    struct Person{
        name:String,
        age:u8,
    }

    let person = Person{
        name:"lanshi".to_string(),
        age:24,
    };
    let Person { name,ref age } = person;
    println!("name:{}",name);
    println!("age:{}",age);

    // println!("{}",person.name);
    println!("{}",person.age);

}