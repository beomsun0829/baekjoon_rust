use std::collections::HashMap;
use std::io;
use std::fmt::Write;

fn get_line() -> String{
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn main(){
    let mut answer = String::new();

    let line: Vec<i32> = get_line().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (n, m) = (line[0], line[1]);

    let mut passwords: HashMap<String, String> = HashMap::new();

    for _ in 0..n{
        let line: Vec<String> = get_line().trim().split_whitespace().map(|s| s.to_string()).collect();
        let site = line[0].to_string();
        let password = line[1].to_string();

        passwords.insert(site, password);
    }

    for _ in 0..m{
        let line = get_line().trim().to_string();
        writeln!(&mut answer, "{}", passwords.get(&line).unwrap());
    }

    println!("{}", answer);

}