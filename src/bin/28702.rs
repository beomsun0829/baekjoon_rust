use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let string1 = get_line();
    let string2 = get_line();
    let string3 = get_line();

    let mut find: i32 = 0;

    if string1.parse::<i32>().is_ok() {
        find = string1.parse::<i32>().unwrap() + 3;
    } else if string2.parse::<i32>().is_ok() {
        find = string2.parse::<i32>().unwrap() + 2;
    } else if string3.parse::<i32>().is_ok() {
        find = string3.parse::<i32>().unwrap() + 1;
    }

    if find % 3 == 0 && find % 5 == 0 {
        println!("FizzBuzz");
    } else if find % 3 == 0 {
        println!("Fizz");
    } else if find % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{find}");
    }
}
