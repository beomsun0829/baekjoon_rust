use std::cmp::min;
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

fn city_chicken(houses: &[(usize, usize)], selected_chickens: &[(usize, usize)]) -> usize {
    let mut total_dist = 0;
    for &(hy, hx) in houses {
        let mut min_dist = usize::MAX;

        for &(cy, cx) in selected_chickens {
            let dist = usize::abs_diff(hy, cy) + usize::abs_diff(hx, cx);
            min_dist = min(dist, min_dist);
        }
        total_dist += min_dist;
    }
    total_dist
}

fn recursive(
    mut selected_chickens: Vec<(usize, usize)>,
    start_idx: usize,
    count: usize,
    m: usize,
    chickens: &Vec<(usize, usize)>,
    houses: &Vec<(usize, usize)>,
    ans: &mut usize,
) {
    if count == m {
        let dist = city_chicken(houses, &selected_chickens);
        *ans = min(*ans, dist);
        return;
    }

    if start_idx >= chickens.len() {
        return;
    }

    selected_chickens.push(chickens[start_idx]);
    recursive(selected_chickens.clone(), start_idx + 1, count + 1, m, chickens, houses, ans);

    selected_chickens.pop();
    recursive(selected_chickens, start_idx + 1, count, m, chickens, houses, ans);
}

fn main() {
    let line = get_vec_line::<usize>();
    let (n, m) = (line[0], line[1]);

    let mut map: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = get_vec_line::<i32>();
        map.push(line);
    }

    let mut chickens: Vec<(usize, usize)> = Vec::new();
    let mut houses: Vec<(usize, usize)> = Vec::new();

    for i in 0..n {
        for j in 0..n {
            if map[i][j] == 1 {
                houses.push((i, j));
            } else if map[i][j] == 2 {
                chickens.push((i, j));
            }
        }
    }

    let mut ans = usize::MAX;
    recursive(Vec::new(), 0, 0, m, &chickens, &houses, &mut ans);
    println!("{ans}");
}
