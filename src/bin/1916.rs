use std::cmp::Reverse;
use std::collections::BinaryHeap;
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

fn main() {
    let n: usize = get_line().parse().unwrap();
    let m: usize = get_line().parse().unwrap();

    let mut matrix: Vec<Vec<(usize, usize)>> = vec![vec![]; n];

    for _ in 0..m {
        let line = get_vec_line::<usize>();

        matrix[line[0] - 1].push((line[1] - 1, line[2]));
    }

    let line = get_vec_line::<usize>();
    let start = line[0] - 1;
    let end = line[1] - 1;

    let mut dist: Vec<usize> = vec![usize::MAX; n];
    let mut pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new(); //(거리, 노드번호)
    dist[start] = 0;
    pq.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] {
            continue;
        }

        if u == end {
            println!("{}", d);
            break;
        }

        for &(v, weight) in &matrix[u] {
            let new_dist = d + weight;
            if dist[v] > new_dist {
                dist[v] = new_dist;
                pq.push(Reverse((new_dist, v)));
            }
        }
    }
}
