use std::fmt::Write;
use std::io;

fn hanoi(number: i32, src: i32, dest: i32, sub: i32, output: &mut String){
    if number == 1 {
        writeln!(output, "{src} {dest}");
        return
    }

    hanoi(number - 1, src, sub, dest, output);
    writeln!(output, "{src} {dest}");
    hanoi(number - 1, sub, dest, src, output);

    return
}

fn string_add_rust_style(a: &str, b: &str) -> String {
    let mut sum = 0;
    let mut result_digits = Vec::new();
    let a_digits: Vec<u8> = a
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let b_digits: Vec<u8> = b
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let max_len = std::cmp::max(a_digits.len(), b_digits.len());

    for i in 0..max_len {
        let mut current_sum = sum;
        if let Some(digit_a) = a_digits.get(i) {
            current_sum += digit_a;
        }
        if let Some(digit_b) = b_digits.get(i) {
            current_sum += digit_b;
        }

        result_digits.push(current_sum % 10);
        sum = current_sum / 10;
    }
    if sum > 0 {
        result_digits.push(sum);
    }
    result_digits.reverse();
    result_digits
        .into_iter()
        .map(|digit| (digit + b'0') as char)
        .collect()
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let n: i32 = input_line.trim().parse().unwrap();

    let mut k_string = String::from("1");

    for _ in 0..n {
        k_string = string_add_rust_style(&k_string, &k_string);
    }

    if n == 0 {
        k_string = String::from("0");
    } else {
        let last_char_index = k_string.len() - 1;
        let last_char = k_string.chars().nth(last_char_index).unwrap();
        let last_digit = last_char.to_digit(10).unwrap();
        let new_last_digit = last_digit - 1;
        let new_last_char = (new_last_digit as u8 + b'0') as char;
        k_string.replace_range(
            last_char_index..=last_char_index,
            &new_last_char.to_string(),
        );
    }

    println!("{}", k_string);

    let mut move_output = String::new();

    if n > 0 && n <= 20 {
        hanoi(n, 1, 3, 2, &mut move_output);
    }

    print!("{}", move_output);
}
