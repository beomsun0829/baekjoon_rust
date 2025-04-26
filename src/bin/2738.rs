use std::io;

fn read_line() -> Vec<i32>{
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read line fail");
    let vec: Vec<i32> = line.trim().split_whitespace().map(|s| s.parse().expect("parse fail")).collect();
    return vec;
}


fn main(){
    let nm = read_line();
    let n: usize = nm[0] as usize;
    let m: usize = nm[1] as usize;

    let mut mat1: Vec<Vec<i32>> = Vec::new();

    for _ in 0..n{
        mat1.push(read_line());
    }

    for i in 0..n{
        let row = read_line();
        for j in 0..m{
            print!("{} ", row[j] + mat1[i][j]);
        }
        println!();
    }
}