use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let n: usize = get_line().parse().unwrap();
    let m: usize = get_line().parse().unwrap();
    let s: Vec<char> = get_line().chars().collect();

    let mut start: usize = 0;
    let mut end: usize = 1;

    let mut ans = 0;
    let size = n * 2 + 1;

    while start < m {
        while end < m {
            let now_len = end - start + 1;

            /*
            println!("{:?}", s);
            println!("{:?}", &s[start..end + 1]);
            println!("{}, {}", start, end);
            println!("{}", now_len);
            println!("{}", ans);
            get_line();
            */
            
            if now_len == size && s[start] == 'I' && s[end] == 'I'{
                ans += 1;
                start += 1;
                continue;
            }

            if now_len > size{
                start += 1;
                continue;
            }

            if s[end] == s[end - 1] {
                start = end;
            }

            end += 1;
        }
        break;
    }

    println!("{ans}");
}
