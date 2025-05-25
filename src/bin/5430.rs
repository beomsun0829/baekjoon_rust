use std::collections::VecDeque;
use std::fmt::Write;
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
    get_line()
        .split_whitespace()
        .filter_map(|x| x.parse::<T>().ok())
        .collect()
}

fn ac(p: &str, x: &mut VecDeque<i32>) -> String {
    let mut reversed: bool = false;

    for c in p.chars() {
        match c {
            'R' => {
                reversed = !reversed;
            }
            'D' => {
                let pop_result = if reversed {
                    x.pop_back()
                } else {
                    x.pop_front()
                };

                if pop_result.is_none() {
                    return String::from("error");
                }
            }
            _ => return String::from("error"),
        }
    }

    let elements: Vec<String> = if reversed {
        x.iter().rev().map(|&val| val.to_string()).collect()
    } else {
        x.iter().map(|&val| val.to_string()).collect()
    };
    format!("[{}]", elements.join(","))
}

fn main() {
    let t: i32 = get_line().parse().unwrap();

    for _ in 0..t {
        let p = get_line();
        let n: i32 = get_line().parse().unwrap();
        let mut x_raw = get_line();
        let mut x: VecDeque<i32> = VecDeque::new();

        x_raw = x_raw[1..x_raw.len() - 1].to_string();
        x = x_raw.split(',').filter_map(|s| s.trim().parse::<i32>().ok()).collect();

        let res = ac(&p, &mut x);
        println!("{res}");
    }
}
