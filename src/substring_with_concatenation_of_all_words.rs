use std::collections::{HashMap, VecDeque};

fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let mut k = words[0].len();
    let mut conditions = HashMap::new();
    for word in words.iter() {
        *conditions.entry(word.clone()).or_insert(0) += 1;
    }

    let mut hashmap_list = HashMap::<usize, (usize, HashMap::<String, usize>)>::new();
    let mut strings = HashMap::<usize, VecDeque<(usize, String)>>::new();
    let mut res = Vec::new();
    let s = s.as_bytes();
    for r in 0..s.len() {
        if r >= k-1 {
            let key = (r+1)%k;
            let value = String::from_utf8(s[(r+1-k)..=r].into()).unwrap();

            if !conditions.contains_key(&value) {
                hashmap_list.remove(&key);
                strings.remove(&key);
                continue;
            }

            let (n, hashmap) = hashmap_list.entry(key).or_insert((0, HashMap::new()));
            hashmap.entry(value.clone()).and_modify(|v| *v += 1).or_insert(1);
            *n += 1;

            let groups = strings.entry(key).or_default();
            groups.push_back((r+1-k, value));

            if *n == words.len() {
                let (idx, key) = groups.pop_front().unwrap();
                if *hashmap == conditions {
                    res.push(idx as _);
                }

                hashmap.entry(key).and_modify(|v| *v -= 1);
                *n -= 1;
            }
        }
    }

    res
}

pub fn main() {
    let s = "barfoofoobarthefoobarman".to_string();
    let words = ["foo","bar", "the"].into_iter().map(String::from).collect::<Vec<_>>();
    // let s = "a".to_string();
    // let words = vec!["a".to_string()];

    println!("{:?}", find_substring(s, words));
}
