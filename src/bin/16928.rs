use std::collections::{HashMap, VecDeque};
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
    let line = get_vec_line::<i32>();
    let (n, m) = (line[0], line[1]);
    let mut shortcuts: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    for _ in 0..n + m {
        let line = get_vec_line::<i32>();
        let from = line[0] - 1;
        let to = line[1] - 1;
        shortcuts.insert((from / 10, from % 10), (to / 10, to % 10));
    }

    let mut board: Vec<Vec<i32>> = vec![vec![1000; 10]; 10];
    board[0][0] = 0;

    let mut deque: VecDeque<(i32, i32, i32)> = VecDeque::new();
    deque.push_back((0, 0, 0));

    let dices = [1, 2, 3, 4, 5, 6];

    while let Some((y, x, dist)) = deque.pop_front() {
        for dice in dices {
            let mut next_y = y;
            let mut next_x = x + dice;

            if next_x >= 10 {
                next_x %= 10;
                next_y += 1;
            }

            if next_x < 10 && next_y < 10 {
                match shortcuts.get(&(next_y, next_x)) {
                    Some(&(short_y, short_x)) => {
                        if board[short_y as usize][short_x as usize] > dist + 1 {
                            board[short_y as usize][short_x as usize] = dist + 1;
                            deque.push_back((short_y, short_x, dist + 1));
                        }
                    }
                    None => {
                        if board[next_y as usize][next_x as usize] > dist + 1 {
                            board[next_y as usize][next_x as usize] = dist + 1;
                            deque.push_back((next_y, next_x, dist + 1));
                        }
                    }
                }
            }
        }
    }

    /*
      for i in 0..10 as usize {
          for j in 0..10 as usize {
              print!("{} ", board[i][j]);
          }
          println!("");
      }

    */

    println!("{}", board[9][9]);
}
