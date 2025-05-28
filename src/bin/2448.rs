use std::io;

fn draw(n: usize) -> Vec<String> {
    if n == 3 {
        return vec![
            "  *  ".to_string(),
            " * * ".to_string(),
            "*****".to_string(),
        ];
    }
    
    let sub = draw(n/2);
    let mut output: Vec<String> = Vec::new();

    let space = " ".repeat(n/2);

    for s in &sub{
        output.push(format!("{}{}{}", space, s, space));
    }

    for s in &sub{
        output.push(format!("{}{}{}", s, " ", s));
    }

    output
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    println!("{}", draw(n).join("\n"));
}
