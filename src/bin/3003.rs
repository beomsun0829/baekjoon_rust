use std::io;

fn main(){
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("read_line fail");

    let v: Vec<i32> = input_line.trim().split_whitespace().map(|s| s.parse::<i32>().expect("")).collect();

    let correct_piece: [i32; 6] = [1,1,2,2,2,8];

    for i in 0..6{
        print!("{} ", correct_piece[i] - v[i]);
    }

}