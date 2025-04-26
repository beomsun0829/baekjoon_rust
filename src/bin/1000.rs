use std::io;

fn main(){
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("read line fail");
    
    let parts: Vec<&str> = input_line.trim().split_whitespace().collect();

    let a: i32 = parts[0].parse().expect("parse fail");
    let b: i32 = parts[1].parse().expect("parse fail");

    println!("{}", a+b);
}