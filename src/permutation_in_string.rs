use std::collections::{HashMap, VecDeque};

fn check_inclusion(s1: String, s2: String) -> bool {
    let mut hashmap_s1 = HashMap::<u8, i32>::new();
    let mut hashmap_s2 = HashMap::<u8, i32>::new();
    let mut window = VecDeque::new();

    for &byte in s1.as_bytes() {
        *hashmap_s1.entry(byte).or_default() += 1;
    }

    for &byte in s2.as_bytes() {
        *hashmap_s2.entry(byte).or_default() += 1;
        window.push_back(byte);

        if window.len() > s1.len() {
            let popped_front = window.pop_front().unwrap();
            let mut val = hashmap_s2.get_mut(&popped_front).unwrap();
            *val -= 1;
            if *val == 0 {
                hashmap_s2.remove(&popped_front);
            }
        }

        if window.len() >= s1.len() && hashmap_s2 == hashmap_s1 {
            return true;
        }
    }

    false
}

pub fn main() {
    let s1 = "ab".to_string();
    let s2 = "eidbaooo".to_string();
    println!("{}", check_inclusion(s1, s2));
}
