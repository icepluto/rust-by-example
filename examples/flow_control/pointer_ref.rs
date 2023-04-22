fn main() {
    let r = &4;
    match r {
        &val => {
            println!("{}", val)
        }
    }
    match *r {
        val => {
            println!("{}", val);
        }
    }
    let ref is_a_ref = 10;
    match is_a_ref{
        &val =>{
            println!("{}",val);
        }
    }
    let value = 5;
    let mut mut_value = 6;
    match value {
        ref val=>{
            println!("{}",val);
        }
    }
    match mut_value{
        ref mut val =>{
            *val+=1;
            println!("{}",val);
        }
    }

}
