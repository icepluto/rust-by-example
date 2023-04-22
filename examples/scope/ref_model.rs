struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';
    let ref ref_c1 = c;
    let ref_c2 = &c;
    //true
    println!("{}", ref_c1 == ref_c2);
    println!("{}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    let copy_of_x = {
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;
        *ref_to_x
    };
    println!("copy of x:{}", copy_of_x);

    let mut mut_point = point;
    let copy_of_y = {
        let Point {
            x: _,
            y: ref mut ref_to_y,
        } = mut_point;
        *ref_to_y = 1
    };
    // println!("point:{} {}", point.x, point.y);
    println!("mut_point:{} {}", mut_point.x, mut_point.y);
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        // 解构 `mutable_tuple` 来改变 `last` 的值。
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);
}
