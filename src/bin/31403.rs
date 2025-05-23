use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let a_str = get_line();
    let b_str = get_line();
    let c_str = get_line();

    let a_num: i32 = a_str.parse().unwrap();
    let b_num: i32 = b_str.parse().unwrap();
    let c_num: i32 = c_str.parse().unwrap();

    println!("{}", a_num + b_num - c_num);

    let ab_concat_str = format!("{}{}", a_str, b_str);
    let ab_concat_num: i32 = ab_concat_str.parse().unwrap();

    println!("{}", ab_concat_num - c_num);
}
