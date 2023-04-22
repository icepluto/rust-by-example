
trait Person{
    fn name(&self)->String;
}

trait Student:Person{
    fn university(&self)->String;
}

trait Programmer{
    fn fav_language(&self)->String;
}

trait CompSciStudent:Student+Programmer{
    fn git_username(&self)->String;
}

fn comp_sci_pro_greeting(student:&dyn CompSciStudent)->String{
    format!("My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
    student.name(),
    student.university(),
    student.fav_language(),
    student.git_username(),

)
}
fn main(){

}