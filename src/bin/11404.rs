use std::{i32::MAX, io};

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

    let mut matrix: Vec<Vec<i32>> = vec![vec![MAX; n]; n];
    for i in 0..n {
        matrix[i][i] = 0;
    }

    for _ in 0..m {
        let line = get_vec_line::<i32>();

        matrix[line[0] as usize - 1][line[1] as usize - 1] = std::cmp::min(line[2], matrix[line[0] as usize - 1][line[1] as usize - 1]);
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if matrix[i][k] != MAX && matrix[k][j] != MAX && matrix[i][j] > matrix[i][k] + matrix[k][j] {
                    matrix[i][j] = matrix[i][k] + matrix[k][j];
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if matrix[i][j] == MAX {
                print!("{} ", 0);
            } else {
                print!("{} ", matrix[i][j]);
            }
        }
        println!();
    }
}
