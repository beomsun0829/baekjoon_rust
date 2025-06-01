use std::{cmp::{max, min}, io};

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
    let line = get_vec_line::<usize>();
    let (n, m, r) = (line[0], line[1], line[2]);

    let items = get_vec_line::<usize>();
    let mut adj: Vec<Vec<usize>> = vec![vec![usize::MAX/3 ; n]; n];
    for i in 0..n{
        adj[i][i] = 0;
    }

    for _ in 0..r {
        let line = get_vec_line::<usize>();
        adj[line[0] - 1][line[1] - 1] = line[2];
        adj[line[1] - 1][line[0] - 1] = line[2];
    }

    for k in 0..n{
        for i in 0..n{
            for j in 0..n{
                if adj[i][j] > adj[i][k] + adj[k][j]{
                    adj[i][j] = adj[i][k] + adj[k][j];
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..n{
        let mut item_sum: usize = 0;

        for j in 0..n{
            if adj[i][j] <= m{
                item_sum += items[j];
            }
        }

        ans = max(ans, item_sum);
    }

    println!("{}", ans);
}

/*

5 5 4
5 7 8 2 3
1 4 5
5 2 4
3 2 3
1 2 3
[0, 3, 6, 5, 7]
[3, 0, 3, 8, 4]
[6, 3, 0, 11, 7]
[5, 8, 11, 0, 12]
[7, 4, 7, 12, 0]
*/