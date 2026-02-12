use std::collections::{HashMap, VecDeque};

fn length_of_longest_substring_two_distinct(s: String) -> i32 {
    let mut hashmap = HashMap::<u8, i32>::new();
    let mut window = VecDeque::new();
    let mut count = 0;
    let mut len = 0;
    let mut res = 0;

    for &byte in s.as_bytes() {
        window.push_back(byte);
        len += 1;

        let mut val = hashmap.entry(byte).or_default();
        *val += 1;
        if *val == 1 {
            count += 1;
        }

        while count == 3 {
            let first_value = *window.front().unwrap();
            let mut val = hashmap.get_mut(&first_value).unwrap();
            while let Some(value) = window.pop_front() {
                if value != first_value {
                    window.push_front(value);
                    break;
                } else {
                    *val -= 1;
                    len -= 1;
                }
            }

            if *val == 0 {
                count -= 1;
            }
        }

        if count <= 2 {
            res = res.max(len);
        }
    }

    res
}

pub fn main() {
    let s = "edccccdddeeeeeee".to_string();
    println!("{}", length_of_longest_substring_two_distinct(s));
}
