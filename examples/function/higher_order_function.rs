fn main() {
    println!("find the sum of all the squared odd numbers under 1000");
    let upper = 1000;
    let mut acc = 0;
    for n in 0.. {
        let n_square = n * n;
        if n_square >= upper {
            break;
        } else if is_odd(n_square) {
            acc += n_square
        }
    }
    println!("imperative style: {}", acc);
    let sum_of_odd_squared = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);

    println!("{}",sum_of_odd_squared);
}
fn is_odd(num: u32) -> bool {
    num % 2 == 1
}
