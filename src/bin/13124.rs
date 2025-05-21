use std::io;
use std::fmt::Write;

fn main() {
    let mut input_line: String = String::new();

    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();
    let n: usize = input_line.trim().parse().unwrap();

    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut a: Vec<i32> = input_line
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap() - 1)
        .collect();

    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();
    let q: i32 = input_line.trim().parse().unwrap();

    let mut output: String = String::new();

    for _ in 0..q {
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut parts = input_line.trim().split_whitespace();

        let x: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let y: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;

        a.swap(x, y);

        let res = checker(&a, n);
        writeln!(output, "{res}").unwrap();
    }
    println!("{output}");
}

fn checker(a: &Vec<i32>, n: usize) -> &'static str {
    if (a[0] == 1 && a[1] == (n - 1) as i32 && a[2] == 0)
        || (a[0] == 2 && a[1] == 0 && a[n - 1] == 1)
        || (a[n - 1] == (n - 3) as i32 && a[n - 2] == (n - 1) as i32 && a[0] == (n - 2) as i32)
        || (a[n - 1] == (n - 2) as i32 && a[n - 2] == 0 && a[n - 3] == (n - 1) as i32)
    {
        "YES"
    } else {
        "NO"
    }
}
