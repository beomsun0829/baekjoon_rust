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

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

fn main() {
    let t: i32 = get_line().parse().unwrap();
    for _ in 0..t {
        let parts: Vec<i32> = get_vec_line::<i32>();
        let (m,n,x,y) = (parts[0], parts[1], parts[2], parts[3]);

        let end = lcm(m, n);

        let mut i = x;
        let mut ans = -1;

        while i <= end{
            let mut ny = i % n;
            if ny == 0{
                ny = n;
            }

            if ny == y{
                ans = i;
                break;
            }

            i += m;
        }

        println!("{ans}");
    }
}

/*

*/
