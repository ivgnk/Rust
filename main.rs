use std::slice;
use std::str;
use std::string;
fn main() {
    // https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
    // print!("{}[2J", 27 as char);
    println!("Hello, world!");
    // let age: u32;
    // age = 36;
    // println!("Age = {}", age);
    let age = 36;
    let name = "Tom";
    println!("Name = {}  Age = {}", age, name);
    println!("-----------------");
    let mut age = 36;
    println!("Начальное значение: {}", age);
    age = 25;
    println!("Конечное значение: {}", age);

    println!("-----------------");
    let a: String;
    a = "hello";
    println!("a: {}", a);
}
