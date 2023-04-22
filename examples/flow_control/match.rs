#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    //loop 返回值
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    //match
    let a_number = 13;
    match a_number {
        1 => println!("a_number is 1"),
        2 | 3 | 4 | 5 | 6 => println!("a_number is 2~6"),
        6..=13 => println!("7~13"),
        _ => println!("error"),
    }
}
