use std::fmt::Write;
use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let mut output = String::new();

    let line: Vec<i32> = get_line().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n: i32 = line[0];
    let m: i32 = line[1];

    let nums: Vec<i32> = get_line().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut prefix_sum: Vec<i32> = nums.clone();
    for i in 1..n as usize {
        prefix_sum[i] = prefix_sum[i - 1] + nums[i];
    }
    prefix_sum.insert(0, 0);

    for _ in 0..m {
        let line: Vec<usize> = get_line().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let start_idx: usize = line[0] - 1;
        let end_idx: usize = line[1];

        let res = prefix_sum[end_idx] - prefix_sum[start_idx];

        writeln!(&mut output, "{res}");
    }

    print!("{output}");
}
