use std::collections::{HashMap, VecDeque};

fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
    let mut hashmap = HashMap::<u8, i32>::new();
    let mut window = VecDeque::new();
    let mut amount = 0;
    let mut res = 0;

    for &byte in s.as_bytes() {
        window.push_back(byte);
        let val = hashmap.entry(byte).or_default();
        *val += 1;
        if *val == 1 {
            amount += 1;
        }


        while amount > k {
            let front = *window.front().unwrap();
            let val = hashmap.get_mut(&front).unwrap();
            while let Some(popped_front) = window.pop_front() {
                if popped_front != front {
                    window.push_front(popped_front);
                    break;
                }

                *val -= 1;
            }

            if *val == 0 {
                amount -= 1;
            }
        }

        if amount <= k {
            res = res.max(window.len());
        }
    }

    res as i32
}

pub fn main() {
    let s = "eceba".to_string();
    let k = 2;
    println!("{}", length_of_longest_substring_k_distinct(s, k));
}
