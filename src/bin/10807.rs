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

fn main(){
    let n: usize = get_line().parse().unwrap();
    let v: Vec<i32> = get_vec_line::<i32>();
    let find: i32 = get_line().parse().unwrap();

    let ans = v.iter().filter(|&&el| el == find).count();

    println!("{ans}");
}