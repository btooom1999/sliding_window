use std::collections::{HashMap, VecDeque};

fn length_of_longest_substring(s: String) -> i32 {
    let mut hashmap = HashMap::<u8, i32>::new();
    let mut window = VecDeque::new();
    let mut errors = 0;
    let mut res = 0;
    let mut k = 1;
    let mut n = 0;

    for (i, &byte) in s.into_bytes().iter().enumerate() {
        window.push_back(byte);
        n += 1;

        let value = hashmap.entry(byte).or_default();
        *value += 1;

        if *value == 2 {
            errors += 1;
        }

        if n > k {
            let value = hashmap.get_mut(&window.pop_front().unwrap()).unwrap();
            *value -= 1;
            n -= 1;

            if *value == 1 {
                errors -= 1;
            }
        }

        if errors == 0 {
            res += 1;
            k += 1;
        }

    }

    res
}

pub fn main() {
    let s = "pwwkew".to_string();
    println!("{}", length_of_longest_substring(s));
}
