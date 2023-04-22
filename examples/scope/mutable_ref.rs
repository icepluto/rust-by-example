#[allow(dead_code)]
#[derive(Copy,Clone)]
struct Book{
    author:&'static str,
    title:&'static str,
    year:u32,
}
fn borrow_book(book:&Book){
    println!("lanshi borrowed {} - {} year",book.title,book.year);
}
fn editor_year(book:&mut Book){
    book.year = 2022;
    println!("{} year changed - {}",book.title,book.year);
}

fn main(){
    let book = Book{
        author:"libai",
        title:"jingyesi",
        year:345,
    };
    let mut newbook = book;
    borrow_book(&book);
    borrow_book(&newbook);
    editor_year(&mut newbook);
}