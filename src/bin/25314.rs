use std::io;

fn main(){
    let mut input_line: String = String::new();
    io::stdin().read_line(&mut input_line).expect("line read fail");

    let n: i32 = input_line.trim().parse().expect("parse fail");
    let num_longs: i32 = n/4;
    
    for _ in 0..num_longs{
        print!("long ");
    }
    
    print!("int");
}