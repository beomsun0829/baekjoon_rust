use std::collections::VecDeque;
use std::fmt::Write;
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
    let nm = get_vec_line::<i32>();
    let n = nm[0] as usize;
    let m = nm[1] as usize;

    let mut start_x: usize = 0;
    let mut start_y: usize = 0;
    let mut map: Vec<Vec<i32>> = Vec::with_capacity(n);
    let mut ansmap: Vec<Vec<i32>> = vec![vec![-1; m]; n];

    for i in 0..n {
        let line = get_vec_line::<i32>();

        for j in 0..m {
            if line[j] == 2 {
                start_x = i;
                start_y = j;
                ansmap[i][j] = 0;
            } else if line[j] == 0 {
                ansmap[i][j] = 0;
            }
        }
        map.push(line);
    }

    let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
    queue.push_back((start_x, start_y, 1));

    let dir = [[0, 1], [1, 0], [0, -1], [-1, 0]];

    while let Some((x, y, dist)) = queue.pop_front() {
        for [dx, dy] in dir {
            let next_x = x as i32 + dx;
            let next_y = y as i32 + dy;

            if next_x >= 0 && next_y >= 0 && next_x < n as i32 && next_y < m as i32 {
                let next_x_u = next_x as usize;
                let next_y_u = next_y as usize;

                if map[next_x_u][next_y_u] == 1 && ansmap[next_x_u][next_y_u] == -1 {
                    ansmap[next_x_u][next_y_u] = dist;
                    queue.push_back((next_x_u, next_y_u, dist + 1));
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            print!("{} ", ansmap[i][j]);
        }
        println!("");
    }
}
