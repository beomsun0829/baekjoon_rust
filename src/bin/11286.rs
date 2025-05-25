use std::fmt::Write;
use std::io;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let mut output = String::new();
    let n: i32 = get_line().parse().unwrap();

    let mut min_heap_pos: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut min_heap_neg: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for _ in 0..n {
        let x: i32 = get_line().parse().unwrap();

        if x == 0 {
            let top_pos = min_heap_pos.peek().map(|&Reverse(v)| v);
            let top_neg_abs = min_heap_neg.peek().map(|&Reverse(v)| v);

            if top_pos.is_none() && top_neg_abs.is_none() {
                writeln!(output, "0").unwrap();
            } else if top_pos.is_none() {
                min_heap_neg.pop();
                writeln!(output, "{}", top_neg_abs.unwrap() * -1).unwrap();
            } else if top_neg_abs.is_none() {
                min_heap_pos.pop();
                writeln!(output, "{}", top_pos.unwrap()).unwrap();
            } else {
                if top_pos < top_neg_abs {
                    min_heap_pos.pop();
                    writeln!(output, "{}", top_pos.unwrap()).unwrap();
                } else {
                    min_heap_neg.pop();
                    writeln!(output, "{}", top_neg_abs.unwrap() * -1).unwrap();
                }
            }
        } else if x > 0 {
            min_heap_pos.push(Reverse(x));
        } else {
            min_heap_neg.push(Reverse(-x));
        }
    }
    println!("{}", output);
}
