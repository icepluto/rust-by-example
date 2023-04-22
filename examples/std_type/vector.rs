fn main(){
    let collect_vec:Vec<i32> = (0..10).collect();
    let mut v = vec![1i32,2,3];

    for i in collect_vec{
        print!("{},",i);
    }
    println!("");
    v.push(4);
    for i in &v {
        println!("for in v {},",i);
    }
    println!("{:?}",v);

    println!(" ");
    for i in v.iter(){
        println!("{}",i);
    }

    for (i,x) in v.iter().enumerate(){
        println!("{} {}",i,x);
    }

    for (i,x) in v.iter_mut().enumerate(){
        println!("{} {}",i+1,x);
    }

}