use std::collections::{HashMap, VecDeque};

fn character_replacement(s: String, k: i32) -> i32 {
    let mut hashmap = HashMap::<char, i32>::new();
    let mut window = VecDeque::<(char, i32)>::new();
    let mut res = 0;
    let mut allow = k;
    let mut len = 0;

    for (r, byte) in s.as_bytes().iter().enumerate() {
        let byte = *byte as char;
        len += 1;

        *hashmap.entry(byte).or_default() += 1;

        if window.back().is_some_and(|&x| x.0 == byte) {
            let value = window.back_mut().unwrap();
            *value = (value.0, value.1 + 1);
        } else {
            window.push_back((byte, 1));
        }


        if let Some(&front) = window.front() && byte != front.0 {
            allow -= 1;
        }


        while allow < 0 {
            let (char, count) = window.pop_front().unwrap();
            len -= count;
            *hashmap.get_mut(&char).unwrap() -= count;

            allow = k - (len - hashmap.get(&window.front().unwrap().0).unwrap());
        }


        res = res.max(len + std::cmp::min(allow, r as i32 - len + 1));
    }

    while let Some((char, count)) = window.pop_front() {
        len -= count;
        *hashmap.get_mut(&char).unwrap() -= count;

        let allow = if let Some(&front) = window.front() {
            k - (len - hashmap.get(&window.front().unwrap().0).unwrap())
        } else {
            0
        };
        res = res.max(len + std::cmp::min(allow, s.len() as i32 - len));
    }

    res
}

pub fn main() {
    let s=  "AAAA".to_string();
    let k = 2;
    // let s = "ABAACCDDCAAAAAAAA".to_string();
    // let k = 3;
    println!("{}", character_replacement(s, k));
}
