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
    let line = get_line();
    let mut result = String::new();

    for c in line.chars() {
        let c_u8 = c as u8;

        if b'a' <= c_u8 && c_u8 <= b'z' {
            result.push((c_u8 + b'A' - b'a') as char);
        } else if b'A' <= c_u8 && c_u8 <= b'Z' {
            result.push((c_u8 + b'a' - b'A') as char);
        }
    }

    println!("{}", result);
}
