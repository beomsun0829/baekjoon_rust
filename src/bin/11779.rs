use std::{cmp::Reverse, collections::BinaryHeap, io};

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

fn main() {
    let n: usize = get_line().parse().unwrap();
    let m: usize = get_line().parse().unwrap();

    let mut adj_list: Vec<Vec<(usize, usize)>> = vec![vec![]; n]; //비용, 도착지

    for _ in 0..m {
        let line = get_vec_line::<usize>();
        adj_list[line[0] - 1].push((line[2], line[1] - 1));
    }

    let line = get_vec_line::<usize>();
    let (start, end) = (line[0] - 1, line[1] - 1);

    let mut pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    let mut dist: Vec<usize> = vec![usize::MAX / 3; n];
    let mut prev: Vec<Option<usize>> = vec![None; n];

    dist[start] = 0;
    pq.push(Reverse((0, start)));

    while let Some(Reverse((d, v))) = pq.pop() {
        /*
        println!("{:?}", pq);
        println!("{} {} {:?}\n", d, v, trace);
        */
        if dist[v] < d {
            continue;
        }

        for &(cost, next_city) in &adj_list[v] {
            let new_cost = d + cost;
            if new_cost < dist[next_city] {
                dist[next_city] = new_cost;
                prev[next_city] = Some(v);
                pq.push(Reverse((new_cost, next_city)));
            }
        }
    }

    /*
    println!("{:?}", dist);
    println!("{:?}", min_trace);
    */

    println!("{}", dist[end]);

    let mut path: Vec<usize> = Vec::new();
    let mut current = end;

    while let Some(p) = prev[current]{
        path.push(current);
        current = p;
        if current == start{
            path.push(start);
            break;
        }
    }

    path.reverse();

    println!("{}", path.len());
    for &node in &path{
        print!("{} ", node + 1);
    }
}
