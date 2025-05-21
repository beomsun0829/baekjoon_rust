use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn dfs(start_idx: usize, n: usize, m: usize, nums: &mut Vec<usize>, answer: &mut Vec<usize>) {
    if answer.len() == m {
        for a in answer.iter() {
            print!("{} ", a);
        }
        println!("");
        return
    }

    let mut prev_val = 0;

    for i in start_idx..n {
        if nums[i] == prev_val {
            continue;
        }

        answer.push(nums[i]);
        prev_val = nums[i];

        dfs(i, n, m, nums, answer);
        answer.pop();
    }
}

fn main() {
    let line: Vec<usize> = get_line()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let (n, m) = (line[0], line[1]);

    let mut nums: Vec<usize> = get_line()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    nums.sort();

    let mut answer: Vec<usize> = Vec::new();

    dfs(0, n, m, &mut nums, &mut answer);
}
