use std::collections::HashSet;
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

type Coord = (i32, i32);
type BlockShape = Vec<Coord>;

fn main() {
    let line = get_vec_line::<usize>();
    let (n, m) = (line[0], line[1]);

    let mut map: Vec<Vec<i32>> = Vec::with_capacity(n);

    for _ in 0..n {
        let line = get_vec_line::<i32>();
        map.push(line);
    }

    let base_tetris: Vec<BlockShape> = vec![
        vec![(0, 0), (1, 0), (2, 0), (2, 1)], // L
        vec![(0, 0), (1, 0), (1, 1), (2, 1)], // 계단
        vec![(0, 0), (0, 1), (0, 2), (0, 3)], // 일자
        vec![(0, 0), (0, 1), (1, 0), (1, 1)], // 네모
        vec![(0, 0), (0, 1), (1, 1), (0, 2)], // T
    ];

    let mut tetris: HashSet<BlockShape> = HashSet::new();
    for base_block in base_tetris {
        let mut current_block = base_block.clone();

        for _ in 0..4 {
            tetris.insert(current_block.clone());
            tetris.insert(current_block.iter().map(|&(r, c)| (r, -c)).collect());
            tetris.insert(current_block.iter().map(|&(r, c)| (-r, c)).collect());
            current_block = current_block.iter().map(|&(r, c)| (c, -r)).collect();
        }
    }

    let mut ans = 0;

    for i in 0..n {
        for j in 0..m {
            for block in &tetris {
                let mut sum = 0;    

                for &part in block {
                    let y = part.0 + i as i32;
                    let x = part.1 + j as i32;
                    if y >= 0 && x >= 0 && y < n as i32 && x < m as i32 {
                        sum += map[y as usize][x as usize];
                    } else {
                        sum = -1;
                        break;
                    }
                }

                if sum >= 0 {
                    ans = std::cmp::max(ans, sum);
                }
            }
        }
    }

    println!("{ans}");
}
