use std::collections::{HashMap, VecDeque};

fn length_of_longest_substring_two_distinct(s: String) -> i32 {
    let mut hashmap = HashMap::<u8, i32>::new();
    let mut window = VecDeque::new();
    let mut res = 0;

    for &byte in s.as_bytes() {
        window.push_back(byte);

        let mut val = hashmap.entry(byte).or_default();
        *val += 1;

        while hashmap.len() == 3 {
            let first_value = *window.front().unwrap();
            let mut val = hashmap.get_mut(&first_value).unwrap();
            while let Some(value) = window.pop_front() {
                if value != first_value {
                    window.push_front(value);
                    break;
                }

                *val -= 1;
            }

            if *val == 0 {
                hashmap.remove(&first_value);
            }
        }

        res = res.max(window.len());
    }

    res as i32
}

pub fn main() {
    let s = "edeeecdddeeeeeee".to_string();
    println!("{}", length_of_longest_substring_two_distinct(s));
}
