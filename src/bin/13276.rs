use std::{collections::HashSet, io};

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn lps(chars: &Vec<char>) -> Vec<usize> {
    let chars_len = chars.len();

    let mut lps: Vec<usize> = vec![0; chars_len];
    let mut length = 0;

    for i in 1..chars_len {
        while length > 0 && chars[i] != chars[length] {
            length = lps[length - 1];
        }

        if chars[i] == chars[length] {
            length += 1;
            lps[i] = length;
        }
    }

    lps
}

fn kmp(parent: &Vec<char>, pattern: &Vec<char>) -> Vec<usize> {
    let lps = lps(pattern);
    let mut length: usize = 0;

    let parent_len = parent.len();
    let pattern_len = pattern.len();
    let mut res: Vec<usize> = vec![0; parent_len];

    for i in 0..parent_len {
        while length > 0 && parent[i] != pattern[length] {
            length = lps[length - 1];
        }

        if parent[i] == pattern[length] {
            length += 1;
        }

        if length == pattern_len {
            res[i + 1 - pattern_len] = 1;
            length = lps[length - 1];
        }
    }

    res
}

const P1: u64 = 31;
const M1: u64 = 1_000_000_007;
const P2: u64 = 37;
const M2: u64 = 1_000_000_009;

fn precompute_hashes(s: &Vec<char>) -> (Vec<u64>, Vec<u64>, Vec<u64>, Vec<u64>) {
    let n = s.len();
    let mut hash1: Vec<u64> = vec![0; n + 1];
    let mut hash2: Vec<u64> = vec![0; n + 1];
    let mut p_pow1: Vec<u64> = vec![0; n + 1];
    let mut p_pow2: Vec<u64> = vec![0; n + 1];

    p_pow1[0] = 1;
    p_pow2[0] = 1;

    for i in 0..n {
        p_pow1[i + 1] = (p_pow1[i] * P1) % M1;
        p_pow2[i + 1] = (p_pow2[i] * P2) % M2;

        hash1[i + 1] = (hash1[i] * P1 + (s[i] as u64 + 1)) % M1;
        hash2[i + 1] = (hash2[i] * P2 + (s[i] as u64 + 1)) % M2;
    }
    (hash1, hash2, p_pow1, p_pow2)
}

fn get_substring_hash(start: usize, end: usize, hash1: &Vec<u64>, hash2: &Vec<u64>, p_pow1: &Vec<u64>, p_pow2: &Vec<u64>) -> (u64, u64) {
    let len = end - start + 1;

    let term1_1 = hash1[end + 1];
    let term2_1 = (hash1[start] * p_pow1[len]) % M1;
    let h1 = (term1_1 + M1 - term2_1) % M1;

    let term1_2 = hash2[end + 1];
    let term2_2 = (hash2[start] * p_pow2[len]) % M2;
    let h2 = (term1_2 + M2 - term2_2) % M2;

    (h1, h2)
}

fn main() {
    let s: Vec<char> = get_line().chars().collect();
    let a: Vec<char> = get_line().chars().collect();
    let b: Vec<char> = get_line().chars().collect();

    let s_len = s.len();
    let a_len = a.len();
    let b_len = b.len();

    let a_start = kmp(&s, &a);
    let b_start = kmp(&s, &b);

    let (hash1_s, hash2_s, p_pow1_s, p_pow2_s) = precompute_hashes(&s);
    let mut substrings: HashSet<(u64, u64)> = HashSet::new();

    for a_start_idx in 0..s_len {
        if a_start[a_start_idx] == 1 {
            for b_start_idx in 0..s_len {
                if b_start[b_start_idx] == 1 {
                    let substring_end_idx = b_start_idx + b_len - 1;

                    let substring_len = substring_end_idx as isize - a_start_idx as isize + 1;

                    if substring_end_idx >= s_len {
                        continue;
                    }
                    if substring_len < a_len as isize || substring_len < b_len as isize || a_start_idx > b_start_idx {
                        continue;
                    }

                    let sub_hash = get_substring_hash(a_start_idx, substring_end_idx, &hash1_s, &hash2_s, &p_pow1_s, &p_pow2_s);
                    substrings.insert(sub_hash);
                }
            }
        }
    }

    println!("{}", substrings.len());
}
