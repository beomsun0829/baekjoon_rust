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

fn fruit_kinds(fruit_count: &Vec<i32>) -> i32 {
    let mut count = 0;
    for f in fruit_count {
        if *f > 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let n: usize = get_line().parse().unwrap();
    let fruits: Vec<usize> = get_vec_line::<usize>();

    let mut start: usize = 0;
    let mut end: usize = 0;

    let mut fruit_count: Vec<i32> = vec![0; 10];
    let mut ans: usize = 0;

    while start < n {
        while end < n {
            let now_fruit = fruits[end];
            fruit_count[now_fruit] += 1;

            if fruit_kinds(&fruit_count) > 2 {
                fruit_count[fruits[start]] -= 1;
                start += 1;
            }

            if (end - start + 1) > ans {
                ans = end - start + 1;
            }

            end += 1;

            /*
            println!("{:?}", fruit_count);
            println!("{:?}", fruits);
            println!("{}, {}", start, end);
            */
        }
        break;
    }

    println!("{ans}");
}
