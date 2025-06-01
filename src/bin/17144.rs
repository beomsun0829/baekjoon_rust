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

fn spread(r: usize, c: usize, map: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut newmap = vec![vec![0; c]; r];
    let dy: [i32; 4] = [0, 0, 1, -1];
    let dx: [i32; 4] = [1, -1, 0, 0];

    for i in 0..r {
        for j in 0..c {
            if map[i][j] > 0 {
                newmap[i][j] += map[i][j];
                let mut dust_count = 0;

                for k in 0..4 {
                    let next_y = i as i32 + dy[k];
                    let next_x = j as i32 + dx[k];

                    if next_y >= 0 && next_x >= 0 && next_y < r as i32 && next_x < c as i32 && map[next_y as usize][next_x as usize] != -1 {
                        newmap[next_y as usize][next_x as usize] += map[i][j] / 5;
                        dust_count += map[i][j] / 5;
                    }
                }

                newmap[i][j] -= dust_count;
            }
        }
    }

    newmap
}

fn run_purifier(r: usize, c: usize, map: &Vec<Vec<i32>>, purifier: usize) -> Vec<Vec<i32>> {
    let mut newmap = map.clone();
    let purifier_top = purifier - 1;
    let purifier_low = purifier;

    for j in 2..c {
        newmap[purifier_top][j] = map[purifier_top][j - 1];
        newmap[purifier_low][j] = map[purifier_low][j - 1];
    }
    newmap[purifier_top][1] = 0;
    newmap[purifier_low][1] = 0;

    for i in 0..purifier_top{
        newmap[i][c-1] = map[i+1][c-1];
    }
    for i in purifier_low + 1..r{
        newmap[i][c-1] = map[i-1][c-1];
    }

    for j in 0..c-1{
        newmap[0][j] = map[0][j+1];
        newmap[r-1][j] = map[r-1][j+1];
    }

    for i in 1..purifier_top{
        newmap[i][0] = map[i-1][0];
    }
    for i in purifier_low+1..r-1{
        newmap[i][0] = map[i+1][0];
    }

    newmap
}

fn main() {
    let line = get_vec_line::<usize>();
    let (r, c, t) = (line[0], line[1], line[2]);
    let mut map: Vec<Vec<i32>> = Vec::with_capacity(r);

    let mut purifier: usize = 0;

    for i in 0..r {
        let line = get_vec_line::<i32>();
        map.push(line);

        for j in 0..c {
            if map[i][j] == -1 {
                purifier = i;
            }
        }
    }

    let mut time = 0;
    while time < t {
        time += 1;

        map = spread(r, c, &map);
        map[purifier - 1][0] = -1;
        map[purifier][0] = -1;

        map = run_purifier(r, c, &mut map, purifier);

        if time == t {
            let mut dusts = 0;
            for i in 0..r {
                for j in 0..c {
                    if map[i][j] > 0 {
                        dusts += map[i][j];
                    }
                }
            }

            println!("{dusts}");
            break;
        }
    }
}
