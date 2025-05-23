use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let str = get_line();
    let idx: usize = get_line().parse().unwrap();

    println!("{}", str.chars().nth(idx - 1).unwrap());
}
