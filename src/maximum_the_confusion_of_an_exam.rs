use std::collections::{HashMap, VecDeque};

fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    let mut count = k;
    let mut hashmap = HashMap::new();
    let mut window = VecDeque::new();
    let mut res = 0;
    let mut total = 0;

    for (r, &key) in answer_key.as_bytes().iter().enumerate() {
        let r = r as i32 + 1;
        total += 1;
        *hashmap.entry(key).or_insert(0) += 1;

        if window.back().is_some_and(|(prev_key, _)| *prev_key == key) {
            let val = window.back_mut().unwrap();
            *val = (val.0, val.1 + 1);
        } else {
            window.push_back((key, 1_i32));
        }

        if let Some(front) = window.front() && front.0 != key {
            count -= 1;
        }

        while count < 0 {
            let (key, value) = window.pop_front().unwrap();
            *hashmap.get_mut(&key).unwrap() -= value;
            total -= value;

            count = k - (total - hashmap.get(&window.front().unwrap().0).unwrap());
        }

        res = res.max(total + std::cmp::min(count, r - total));
    }

    while let Some((key, value)) = window.pop_front() {
        total -= value;
        *hashmap.get_mut(&key).unwrap() -= value;

        let count = if let Some(&(key, _)) = window.front() {
            k - (total - hashmap.get(&key).unwrap())
        } else {
            0
        };
        res = res.max(total + std::cmp::min(count, answer_key.len() as i32 - total));
    }

    res
}

pub fn main() {
    let answer_key = "F".to_string();
    let k = 1;
    println!("{}", max_consecutive_answers(answer_key, k));
}
