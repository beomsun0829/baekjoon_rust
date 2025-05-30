use std::{cmp::max, collections::VecDeque, io};

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

const DY: [i32; 4] = [1, -1, 0, 0];
const DX: [i32; 4] = [0, 0, 1, -1];
fn bfs(n: i32, m: i32, mut map: Vec<Vec<usize>>, initial_virus: &Vec<(usize, usize)>) -> usize {
    let mut dq: VecDeque<(usize, usize)> = initial_virus.clone().into();

    while let Some((y, x)) = dq.pop_front() {
        for d in 0..4 {
            let next_y: i32 = y as i32 + DY[d];
            let next_x: i32 = x as i32 + DX[d];

            if next_y >= 0 && next_y < n && next_x >= 0 && next_x < m {
                let next_y_u = next_y as usize;
                let next_x_u = next_x as usize;

                if map[next_y_u][next_x_u] == 0 {
                    map[next_y_u][next_x_u] = 2;
                    dq.push_back((next_y_u, next_x_u));
                }
            }
        }
    }

    let mut res: usize = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i as usize][j as usize] == 0 {
                res += 1;
            }
        }
    }
    res
}
fn dfs_build_wall(
    count: usize,
    start_idx: usize,
    n: usize,
    m: usize,
    map: &mut Vec<Vec<usize>>,
    initial_virus: &Vec<(usize, usize)>,
    max_safe_area: &mut usize,
) {
    if count == 3 {
        let map_clone = map.clone();
        let safe_area = bfs(n as i32, m as i32, map_clone, &initial_virus);
        *max_safe_area = max(safe_area, *max_safe_area);

        /*
        println!("now map: {:?}", map);
        println!("safe_area: {:?}", safe_area);
        println!("max_safe_area: {:?}", *max_safe_area);
        get_line();
        */
        return;
    }

    for i in start_idx..(n * m) {
        let y = i / m;
        let x = i % m;

        if map[y][x] == 0 {
            map[y][x] = 1;
            dfs_build_wall(count + 1, i + 1, n, m, map, &initial_virus, max_safe_area);
            map[y][x] = 0;
        }
    }
}

fn main() {
    let line = get_vec_line::<usize>();
    let (n, m) = (line[0], line[1]);

    let mut map: Vec<Vec<usize>> = Vec::with_capacity(n);
    let mut initial_virus: Vec<(usize, usize)> = Vec::new();

    for i in 0..n {
        let line = get_vec_line::<usize>();
        map.push(line);

        for j in 0..m {
            if map[i][j] == 2 {
                initial_virus.push((i, j));
            }
        }
    }
    let mut max_safe_area: usize = 0;
    dfs_build_wall(0, 0, n, m, &mut map, &initial_virus, &mut max_safe_area);
    println!("{}", max_safe_area);
}
