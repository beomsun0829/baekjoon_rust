use std::cmp::max;
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
    let _n = get_line(); 
    let mut a = get_vec_line::<i32>();

    let _m = get_line(); 
    let mut b = get_vec_line::<i32>();

    let mx = max(*a.iter().max().unwrap_or(&0), *b.iter().max().unwrap_or(&0));
    let mut ans: Vec<i32> = Vec::new();

    for i in (1..=mx).rev() {
        loop {
            let a_idx = a.iter().position(|&x| x == i);
            let b_idx = b.iter().position(|&x| x == i);

            if a_idx.is_none() || b_idx.is_none() {
                break;
            }

            ans.push(i);
            
            a = a.drain(a_idx.unwrap() + 1..).collect();
            b = b.drain(b_idx.unwrap() + 1..).collect();
        }
    }

    println!("{}", ans.len());
    if !ans.is_empty() {
        for val in ans {
            print!("{} ", val);
        }
        println!();
    }
}