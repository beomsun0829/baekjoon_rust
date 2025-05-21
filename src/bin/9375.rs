use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn main() {
    let mut answer = String::new();
    let n: i32 = get_line().trim().parse().unwrap();

    for _ in 0..n {
        let m: i32 = get_line().trim().parse().unwrap();
        let mut clothes: HashMap<String, Vec<String>> = HashMap::new();

        for _ in 0..m {
            let parts: Vec<String> = get_line()
                .trim()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let name = parts[0].clone();
            let category = parts[1].clone();

            match clothes.get_mut(&category) {
                Some(c) => {
                    c.push(name);
                }
                None => {
                    clothes.insert(category, vec![name]);
                }
            }
        }

        let mut result: i32 = 1;
        for (_, c) in &clothes{
            result *= (c.len() as i32) + 1;
        }

        writeln!(&mut answer, "{}", result -1 );

    }

    println!("{answer}");
}
