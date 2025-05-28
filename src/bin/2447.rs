use std::io;

fn draw(n: i32) -> Vec<String> {
    if n == 1 {
        return vec!["*".to_string()];
    }

    let sub = draw(n / 3);
    let mut result: Vec<String> = Vec::new();
    let space = " ".repeat(n as usize / 3);

    for p in &sub {
        result.push(format!("{}{}{}", p, p, p));
    }
    for p in &sub {
        result.push(format!("{}{}{}", p, space, p));
    }
    for p in &sub {
        result.push(format!("{}{}{}", p, p, p));
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    println!("{}", draw(n).join("\n"));
}
