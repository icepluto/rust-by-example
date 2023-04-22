use std::{path::Path, fs::File, io::Read};

fn main(){
    let path = Path::new("hello.txt");
    let display = path.display();
    let mut file = match File::open(&path){
        Err(why) =>{
            panic!("couldn't open {}: {}",display,why);
        },
        Ok(f)=>{
            f
        }
    };
    let mut s = String::new();
    match file.read_to_string(&mut s){
        Err(why)=>{
            panic!("{} {}",display,why);
        },
        Ok(_)=>{
            println!("{} contains: \n {}",display,s);
        },
    }
}