use std::collections::VecDeque;
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

fn do_command(cmd: &str, n: i32) -> i32 {
    match cmd {
        "D" => (n * 2) % 10000,
        "S" => {
            if n == 0 {
                9999
            } else {
                n - 1
            }
        }
        "L" => {
            let front = n / 1000;
            let last = n % 1000;
            last * 10 + front
        }
        "R" => {
            let last = n % 10;
            let front = n / 10;
            last * 1000 + front
        }
        _ => 0,
    }
}

fn main() {
    let t: i32 = get_line().parse().unwrap();
    let commands = ["D", "S", "L", "R"];

    for _ in 0..t {
        let line = get_vec_line::<i32>();
        let (start, end) = (line[0], line[1]);

        let mut visited: Vec<bool> = vec![false; 10001];
        let mut deque: VecDeque<(i32, String)> = VecDeque::new();
        deque.push_back((start, String::new()));

        while let Some((num, s)) = deque.pop_front() {
            for cmd in &commands {
                let res = do_command(cmd, num);

                /*
                println!("{:?}", deque);
                println!("{:?} {:?}", num, s);
                println!("{:?}", cmd);
                println!("{:?}", res);
                get_line();
                */

                if res == end {
                    println!("{}", s.clone() + cmd);
                    deque.clear();
                    break;
                }
                if visited[res as usize] == false {
                    visited[res as usize] = true;
                    deque.push_back((res, s.clone() + cmd));
                }
            }
        }
    }
}
