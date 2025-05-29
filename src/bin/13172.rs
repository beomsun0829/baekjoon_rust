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

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn power(mut base: usize, mut exp: usize, modulus: usize) -> usize {
    let mut res = 1;
    base %= modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            res = res * base % modulus;
        }
        base = base * base % modulus;
        exp /= 2;
    }

    res
}

fn main() {
    const X: usize = 1000000007;

    let m: usize = get_line().parse().unwrap();
    let mut ans = 0;

    for _ in 0..m {
        let line = get_vec_line::<usize>();
        let (n, s) = (line[0], line[1]);

        let g = gcd(n, s);
        let n_g = n / g;
        let s_g = s / g;

        //let b_reciprocal = n_g.pow(X as u32 - 2) % X; //b^-1 = n_g ^ (X-2) mod X

        let b_reciprocal = power(n_g, X - 2, X);
        let q = s_g * b_reciprocal % X; //Q = s_g * b^-1 mod X

        ans += q;
        ans %= X;
    }

    println!("{ans}");
}
