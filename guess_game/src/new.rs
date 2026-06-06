use std::io;

fn main(){
    println!("This is a cool program");


    let mut num = String::new();

    let _ = io::stdin()
    .read_line(&mut num);

    println!("This is the number you just entered, {num}");
}