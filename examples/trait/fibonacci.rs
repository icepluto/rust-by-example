

struct Fibonacci{
    curr:u32,
    next:u32,
}

impl Iterator for Fibonacci{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr+self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}
fn fibonacci()->Fibonacci{
    Fibonacci { curr: 1, next: 1 }
}
fn main(){
    let mut sequence = 0..3;
    println!("{:?}",sequence.next());
    println!("{:?}",sequence.next());
    println!("{:?}",sequence.next());
    println!("{:?}",sequence.next());

    for i in 0..3{
        println!(">{}",i);
    }
    println!("The first ten terms of the Fibonacci sequence are:");
    for i in fibonacci().take(10){
        println!(">{}",i);
    }
    println!("The next 10 terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(10).take(4){
        println!("> {}",i);
    }
    
}