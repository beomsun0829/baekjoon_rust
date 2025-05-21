use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn dfs(
    n: usize,
    m: usize,
    nums: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    answer: &mut Vec<usize>,
) {
    if answer.len() == m {
        for a in answer.iter() {
            print!("{} ", a);
        }
        println!("");
    }

    for i in 0..n {
        if visited[i] {
            continue;
        }
        if i > 0 && nums[i] == nums[i - 1] && !visited[i - 1] {
            continue;
        }

        visited[i] = true;
        answer.push(nums[i]);
        dfs(n, m, nums, visited, answer);
        answer.pop();
        visited[i] = false;
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
    let mut visited: Vec<bool> = vec![false; n];
    let mut answer: Vec<usize> = Vec::new();

    dfs(n, m, &mut nums, &mut visited, &mut answer);
}
