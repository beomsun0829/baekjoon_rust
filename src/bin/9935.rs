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

fn lps(chars: &Vec<char>) -> Vec<usize> {
    let chars_len = chars.len();
    let mut lps: Vec<usize> = vec![0; chars_len];

    let mut length: usize = 0;

    for i in 1..chars_len {
        while length > 0 && chars[i] != chars[length] {
            length = lps[length - 1];
        }

        if chars[i] == chars[length]{
            length += 1;
            lps[i] += length;
        }
    }

    lps
}

fn main() {
    let text: Vec<char> = get_line().chars().collect();
    let bomb: Vec<char> = get_line().chars().collect();
    let bomb_len = bomb.len();

    let lps_bomb: Vec<usize> = lps(&bomb);
    let mut stack: Vec<char> = Vec::new();
    let mut kmp: Vec<usize> = Vec::new();

    for &c in &text {
        stack.push(c);
        let mut pattern_pointer = *kmp.last().unwrap_or(&0);

        while pattern_pointer > 0 && bomb[pattern_pointer] != c {
            pattern_pointer = lps_bomb[pattern_pointer - 1];
        }

        if bomb[pattern_pointer] == c {
            pattern_pointer += 1;
        }

        kmp.push(pattern_pointer);

        if pattern_pointer == bomb_len {
            for _ in 0..bomb_len {
                stack.pop();
                kmp.pop();
            }
        }
    }

    if stack.is_empty() {
        println!("FRULA");
    } else {
        let final_string: String = stack.iter().collect();
        println!("{}", final_string);
    }
}
