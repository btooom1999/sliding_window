use std::collections::HashMap;

fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
    let k = k as _;
    let s = s.as_bytes();
    let mut hashmap = HashMap::new();
    let mut errors = 0;
    let mut res = 0;
    let mut i = 0;
    for j in 0..s.len() {
        let val = hashmap.entry(s[j]).or_insert(0);
        *val += 1;
        if *val == 2 {
            errors += 1;
        }

        if j >= k {
            let val = hashmap.get_mut(&s[i]).unwrap();
            *val -= 1;
            if *val == 1 {
                errors -= 1;
            }
            i += 1;
        }

        if j >= k - 1 && errors == 0 {
            res += 1;
        }
    }

    res
}

pub fn main() {
    let s = "havefunonleetcode".to_string();
    let k = 5;
    // let s = "home".to_string();
    // let k = 5;
    println!("{}", num_k_len_substr_no_repeats(s, k));
}
