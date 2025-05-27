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
    get_line().split_whitespace().filter_map(|x| x.parse::<T>().ok()).collect()
}

fn main() {
    let line = get_vec_line::<i32>();
    let (n, k) = (line[0], line[1]);

    let mx: i32 = 100000;
    let mut points: Vec<i32> = vec![-1; mx as usize + 10];

    let mut deque: VecDeque<(i32, i32)> = VecDeque::new();
    deque.push_back((n, 0));

    while let Some((now, dist)) = deque.pop_front() {
        if now == k {
            println!("{dist}");
            break;
        }

        if now * 2 >= 0 && now * 2 <= mx && points[now as usize * 2] == -1 {
            points[now as usize * 2] = dist;
            deque.push_back((now * 2, dist));
        }
        if now - 1 >= 0 && now - 1 <= mx && points[now as usize - 1] == -1 {
            points[now as usize - 1] = dist + 1;
            deque.push_back((now - 1, dist + 1));
        }
        if now + 1 >= 0 && now + 1 <= mx && points[now as usize + 1] == -1 {
            points[now as usize + 1] = dist + 1;
            deque.push_back((now + 1, dist + 1));
        }
    }
}
