use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn get_vec_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    get_line()
        .split_whitespace()
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

fn dfs(
    x: usize,
    y: usize,
    count: &mut i32,
    map: &mut Vec<Vec<char>>,
    directions: &Vec<(i32, i32)>,
    n: usize,
    m: usize,
) {
    if map[x][y] == 'X' {
        return;
    }

    if map[x][y] == 'P' {
        *count += 1;
    }

    map[x][y] = 'X';
    for (x_dir, y_dir) in directions {
        let next_x = x as i32 + x_dir;
        let next_y = y as i32 + y_dir;
        if next_x >= 0 && next_x < n as i32 && next_y >= 0 && next_y < m as i32 {
            dfs(
                next_x as usize,
                next_y as usize,
                count,
                map,
                directions,
                n,
                m,
            );
        }
    }
}

fn main() {
    let line = get_vec_line::<usize>();
    let (n, m) = (line[0], line[1]);

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut my_x = 0;
    let mut my_y = 0;

    for _ in 0..n {
        let line: Vec<char> = get_line().chars().collect();
        map.push(line);
    }

    for i in 0..n {
        for j in 0..m {
            if map[i as usize][j as usize] == 'I' {
                my_x = i as usize;
                my_y = j as usize;
                map[my_x][my_y] = 'O';
            }
        }
    }

    let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut count = 0;
    dfs(my_x, my_y, &mut count, &mut map, &directions, n, m);

    if count == 0 {
        println!("TT");
    } else {
        println!("{count}");
    }
}
