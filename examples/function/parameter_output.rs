
fn main(){
    let create = create_fn();
    let mut cre_mut = create_mut_fn();
    let once = creat_fnonce();
    create();
    cre_mut();
    once();
}


fn create_fn()->impl Fn(){
    let text = "Fn".to_owned();
    move || println!("this is a {}",text)
}

fn create_mut_fn()->impl FnMut(){
    let text = "FnMut".to_owned();
    move || println!("this is a {}",text)
}
fn creat_fnonce()->impl FnOnce(){
    let text = "FnOnce".to_owned();
    move || println!("this is {}",text)
}