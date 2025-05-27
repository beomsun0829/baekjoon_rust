use std::cmp::{min, Reverse};
use std::collections::{BinaryHeap, VecDeque};
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
    get_line().split_whitespace().filter_map(|x| x.parse::<T>().ok()).collect()
}

fn dijkstra(start_node: usize, n: usize, adj: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let mut dist = vec![usize::MAX; n + 1];
    let mut pq = BinaryHeap::new();

    dist[start_node] = 0;
    pq.push(Reverse((0, start_node)));

    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, weight) in &adj[u] {
            let new_dist = d + weight;

            if new_dist < dist[v] {
                dist[v] = new_dist;
                pq.push(Reverse((new_dist, v)));
            }
        }
    }
    dist
}

fn main() {
    let line = get_vec_line::<usize>();
    let (n, e) = (line[0], line[1]);

    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n + 1];

    for _ in 0..e {
        let edge = get_vec_line::<usize>();
        let (u, v, c) = (edge[0], edge[1], edge[2]);
        adj[u].push((v, c));
        adj[v].push((u, c));
    }

    let must_pass = get_vec_line::<usize>();
    let (v1, v2) = (must_pass[0], must_pass[1]);

    let dist_from_1 = dijkstra(1, n, &adj);
    let dist_from_v1 = dijkstra(v1, n, &adj);
    let dist_from_v2 = dijkstra(v2, n, &adj);

    let mut path1_len = usize::MAX;
    if dist_from_1[v1] != usize::MAX && dist_from_v1[v2] != usize::MAX && dist_from_v2[n] != usize::MAX {
        let d1_v1 = dist_from_1[v1];
        let v1_v2 = dist_from_v1[v2];
        let v2_n = dist_from_v2[n];

        if d1_v1 <= usize::MAX - v1_v2 && d1_v1 + v1_v2 <= usize::MAX - v2_n {
            path1_len = d1_v1 + v1_v2 + v2_n;
        }
    }

    let mut path2_len = usize::MAX;
    if dist_from_1[v2] != usize::MAX && dist_from_v2[v1] != usize::MAX && dist_from_v1[n] != usize::MAX {
        let d1_v2 = dist_from_1[v2];
        let v2_v1 = dist_from_v2[v1];
        let v1_n = dist_from_v1[n];

        if d1_v2 <= usize::MAX - v2_v1 && d1_v2 + v2_v1 <= usize::MAX - v1_n {
            path2_len = d1_v2 + v2_v1 + v1_n;
        }
    }

    let final_min_path = min(path1_len, path2_len);

    if final_min_path == usize::MAX {
        println!("-1");
    } else {
        println!("{}", final_min_path);
    }
}
