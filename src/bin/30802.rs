use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let n: i64 = get_line().parse().unwrap();
    let orders: Vec<i64> = get_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let tp: Vec<i64> = get_line()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let (t, p) = (tp[0], tp[1]);

    let mut t_ans: i64 = 0;

    for order in &orders {
        t_ans += order / t;
        if order % t != 0{
            t_ans += 1;
        }
    }

    println!("{}", t_ans);
    println!("{} {}", n / p, n % p);
}
