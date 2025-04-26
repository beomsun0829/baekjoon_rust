use std::io;

fn main(){
    let mut input_line: String = String::new();
    io::stdin().read_line(&mut input_line).expect("read_line fail");
    let s = input_line.trim();

    let cro_alphabets: [&str; 8] = ["dz=", "c=", "c-", "d-", "lj", "nj", "s=", "z="];

    let mut count: i32 = 0;
    let mut i: usize = 0;

    while i < s.len() {
        let mut matched_len = 1;

        for &alphabet in &cro_alphabets{
            if s[i..].starts_with(alphabet){
                matched_len = alphabet.len();
                break;
            }
        }

        count += 1;
        i += matched_len;
    }

    println!("{}", count);

}