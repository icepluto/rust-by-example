use std::mem;


#[allow(dead_code)]
#[derive(Debug,Clone, Copy)]
struct Point{
    x:f32,
    y:f32,
}

#[allow(dead_code)]
struct Rectangle{
    p1:Point,
    p2:Point,
}

fn origin()->Point{
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin()->Box<Point>{
    Box::new(Point { x: 0.0, y: 0.0})
}

fn main(){
    let point = origin();
    let rectangle = Rectangle{
        p1:origin(),
        p2:Point { x: 3.0, y: 4.0 }
    };

    let boxed_rectangle = Box::new(Rectangle{
        p1:origin(),
        p2:origin(),
    });

    let boxed_point: Box<Point> = Box::new(origin());
    let box_in_box = Box::new(boxed_origin());
    println!("Point occupies {} bytes in the stack",
             mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes in the stack",
             mem::size_of_val(&rectangle));

    // box 的宽度就是指针宽度
    println!("Boxed point occupies {} bytes in the stack",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack",
             mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes in the stack",
             mem::size_of_val(&box_in_box));

    // 将包含在 `boxed_point` 中的数据复制到 `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes in the stack",
             mem::size_of_val(&unboxed_point));

}