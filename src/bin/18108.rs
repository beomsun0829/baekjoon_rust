use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn get_vec_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
{
    get_line().split_whitespace().filter_map(|x| x.parse::<T>().ok()).collect()
}

fn main() {
    let line: i32 = get_line().parse().unwrap();
    println!("{}", line-543);
}