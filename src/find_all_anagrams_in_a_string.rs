fn find_anagrams(s: String, p: String) -> Vec<i32> {
    if s.len() < p.len() {
        return vec![];
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut hashmap = vec![0;26];
    for c in p.chars() {
        hashmap[(c as u8 - b'a') as usize] += 1;
    }

    let mut res = Vec::new();
    for (i, _) in s.iter().enumerate().take(s.len() - p.len() + 1) {
        let mut temp_hashmap = vec![0;26];
        let mut j = i;
        while j < i + p.len() {
            temp_hashmap[(s[j] as u8 - b'a') as usize] += 1;
            j += 1;
        }
        if temp_hashmap == hashmap {
            res.push(i as i32);
        }
    }

    res
}

pub fn main() {
    let s = "cbaebabacd".to_string();
    let p = "abc".to_string();
    println!("{:?}", find_anagrams(s, p));
}
