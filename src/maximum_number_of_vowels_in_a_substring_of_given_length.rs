fn max_vowels(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let k = k as usize;
    let is_vowel_character = |v: u8| [b'a', b'e', b'i', b'o', b'u'].contains(&v);

    let mut count = 0;
    let mut res = 0;
    let mut l = 0;
    for r in 0..s.len() {
        if is_vowel_character(s[r]) {
            count += 1;
        }

        if r >= k {
            if is_vowel_character(s[l]) {
                count -= 1;
            }
            l += 1;
        }

        res = res.max(count);
    }

    res
}

pub fn main() {
    let s = "leetcode".to_string();
    let k = 3;
    println!("{}", max_vowels(s, k));
}
