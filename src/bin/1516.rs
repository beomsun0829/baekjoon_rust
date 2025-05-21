use std::cmp::max;
use std::io;
use std::fmt::Write;

fn get_line() -> String{
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line
}
fn main(){
    let n: usize = get_line().trim().parse().unwrap();
    let mut building: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n{
        building.push(Vec::new());
    }

    let mut build_time: Vec<usize> = vec![0; n];
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];

    let mut count: Vec<i32> = vec![0; n];
    let mut dp: Vec<usize> = vec![0; n];
    let mut queue: Vec<usize> = Vec::new();

    for i in 0..n{
        let mut parts: Vec<i32> = get_line().trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        parts.pop();

        build_time[i] = parts[0] as usize;

        if parts.len() >= 2{
            parts = parts.iter().map(|x| x - 1).collect();
            let build_dependencies = parts[1..parts.len()].to_vec();
            let dep_count = build_dependencies.len();

            for dep in build_dependencies{
                graph[dep as usize].push(i);
            }

            count[i] = dep_count as i32;
        }
        else{
            queue.push(i);
            dp[i] = build_time[i] as usize;
        }
    }

    while !queue.is_empty() {
        let now = queue.remove(0) as usize;

        for dep in graph[now].clone(){
            dp[dep] = max(dp[dep], dp[now] + build_time[dep]);
            count[dep] -= 1;
            
            if count[dep] == 0{
                queue.push(dep);
            }
        }
    }

    for el in dp{
        println!("{el}");
    }

    /*
    println!("{:?} build_time", build_time);
    println!("{:?} graph", graph);
    println!("{:?} count", count);
    println!("{:?} dp", dp);
    println!("{:?} queue", queue);
    */

}
