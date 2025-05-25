use std::collections::VecDeque;
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
    let line = get_vec_line::<usize>();
    let m = line[0];
    let n = line[1];
    let h = line[2];

    let mut tomato: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; m]; n]; h];
    let mut days: usize = 0;
    let mut queue: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();
    let mut left_tomato: i32 = 0;

    for i in 0..h {
        for j in 0..n {
            let line = get_vec_line::<i32>();
            tomato[i][j] = line;
            for k in 0..m {
                if tomato[i][j][k] == 0 {
                    left_tomato += 1;
                }
                if tomato[i][j][k] == 1 {
                    queue.push_back((i, j, k, 0));
                }
            }
        }
    }

    let dir: [[i32; 3]; 6] = [[0, 0, 1], [0, 0, -1], [0, 1, 0], [0, -1, 0], [1, 0, 0], [-1, 0, 0]];

    while let Some((x, y, z, day)) = queue.pop_front() {
        /*
        println!("\n{:?}", queue);
        println!("{:?}", (x, y, z, day));
        println!("{:?}", left_tomato);
        println!("{:?}\n", days);
        */

        days = day;

        for [dx, dy, dz] in dir {
            let next_x = x as i32 + dx;
            let next_y = y as i32 + dy;
            let next_z = z as i32 + dz;

            if next_x >= 0 && next_y >= 0 && next_z >= 0 && next_x < h as i32 && next_y < n as i32 && next_z < m as i32
            {
                let next_x_u = next_x as usize;
                let next_y_u = next_y as usize;
                let next_z_u = next_z as usize;

                if tomato[next_x_u][next_y_u][next_z_u] == 0 {
                    tomato[next_x_u][next_y_u][next_z_u] = 1;
                    left_tomato -= 1;
                    queue.push_back((next_x_u, next_y_u, next_z_u, day + 1));
                }
            }
        }
    }

    if left_tomato == 0 {
        println!("{days}");
    } else {
        println!("-1");
    }
}
