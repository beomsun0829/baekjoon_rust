use std::io;

fn main(){
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read line fail");

    let inputs: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse().expect("parse fail")).collect();
    
    let mut n = inputs[0];
    let b = inputs[1];

    let mut digits: Vec<char> = Vec::new();

    if n == 0{
        digits.push('0');
    }
    else{
        while n > 0{
            let remainder = n % b;
            n = n / b;

            let digit_char = if remainder >= 0 && remainder <= 9{
                (remainder as u8 + b'0') as char
            }
            else{
                (remainder as u8 - 10 + b'A') as char
            };

            digits.push(digit_char);

        }
    }

    digits.reverse();
    for digit in digits{
        print!("{}", digit);
    }
}