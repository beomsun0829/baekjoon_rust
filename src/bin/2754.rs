use std::{collections::HashMap, io};

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
    let score = get_line();

    let mut grade_map: HashMap<&str, f64> = HashMap::new();
    grade_map.insert("A+", 4.3);
    grade_map.insert("A0", 4.0);
    grade_map.insert("A-", 3.7);
    grade_map.insert("B+", 3.3);
    grade_map.insert("B0", 3.0);
    grade_map.insert("B-", 2.7);
    grade_map.insert("C+", 2.3);
    grade_map.insert("C0", 2.0);
    grade_map.insert("C-", 1.7);
    grade_map.insert("D+", 1.3);
    grade_map.insert("D0", 1.0);
    grade_map.insert("D-", 0.7);
    grade_map.insert("F", 0.0);

    println!("{:.1}", grade_map.get(score.as_str()).unwrap());

}
