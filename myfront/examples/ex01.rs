use my_default_macro_derive::MyDefault;

fn main() {
    println!("hello {:?}", User1::default());
    println!("hello {:?}", User2::default());
    println!("hello {:?}", User3::default());
}

#[derive(Debug, MyDefault)]
struct User1 {
    age: u8,
    name: String,
}

#[derive(Debug, MyDefault)]
struct User2(u8, String);

#[derive(Debug, MyDefault)]
struct User3;