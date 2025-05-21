use std::fmt::Write;
use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

struct Node {
    value: usize,
    left: Option<usize>,
    right: Option<usize>,
}

fn char_to_index(c: char) -> usize {
    (c as u8 - b'A') as usize
}

fn index_to_char(u: usize) -> char {
    (u as u8 + b'A') as char
}

fn main() {
    let n: usize = get_line().trim().parse().unwrap();

    let mut tree_nodes: Vec<Node> = Vec::with_capacity(n);
    for i in 0..n {
        tree_nodes.push(Node {
            value: i,
            left: None,
            right: None,
        })
    }

    for _ in 0..n {
        let parts: Vec<char> = get_line()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let now = parts[0];
        let left: Option<usize> = if parts[1] == '.' {
            None
        } else {
            Some(char_to_index(parts[1]))
        };
        let right: Option<usize> = if parts[2] == '.' {
            None
        } else {
            Some(char_to_index(parts[2]))
        };
        let now_idx = char_to_index(now);

        tree_nodes[now_idx].left = left;
        tree_nodes[now_idx].right = right;
    }

    /*
    for node in &tree_nodes {
        println!("{} {:?} {:?}", node.value, node.left, node.right);
    }
    */

    preorder(&tree_nodes, 0);
    println!();
    inorder(&tree_nodes, 0);
    println!();
    postorder(&tree_nodes, 0);
}

fn preorder(tree_nodes: &Vec<Node>, now: usize) {
    print!("{}", index_to_char(now));

    match tree_nodes[now].left {
        Some(left) => preorder(tree_nodes, left),
        None => (),
    }

    match tree_nodes[now].right {
        Some(right) => preorder(tree_nodes, right),
        None => (),
    }
}

fn inorder(tree_nodes: &Vec<Node>, now: usize) {
    match tree_nodes[now].left {
        Some(left) => inorder(tree_nodes, left),
        None => (),
    }

    print!("{}", index_to_char(now));

    match tree_nodes[now].right {
        Some(right) => inorder(tree_nodes, right),
        None => (),
    }
}

fn postorder(tree_nodes: &Vec<Node>, now: usize) {
    match tree_nodes[now].left {
        Some(left) => postorder(tree_nodes, left),
        None => (),
    }

    match tree_nodes[now].right {
        Some(right) => postorder(tree_nodes, right),
        None => (),
    }

    print!("{}", index_to_char(now));
}
