use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let n: i32 = get_line().parse().unwrap();
    if n == 0 {
        println!("0");
        return;
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..n {
        let line: i32 = get_line().parse().unwrap();
        v.push(line);
    }

    v.sort();

    let rmidx: usize = (n as f32 * 0.15).round() as usize;
    let normalized: Vec<i32> = v[rmidx..n as usize - rmidx].to_vec();

    let mut avg: f32 = 0.0;

    for el in &normalized {
        avg += *el as f32;
    }

    let nor_len = normalized.len();
    avg = (avg / nor_len as f32).round();

    println!("{avg}");
}
