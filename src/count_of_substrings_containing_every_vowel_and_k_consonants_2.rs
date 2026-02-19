use std::collections::{HashMap, VecDeque};

fn count_of_substrings(word: String, k: i32) -> i64 {
    let is_vowel = |b: u8| [b'a', b'e', b'i', b'o', b'u'].contains(&b);
    let word = word.as_bytes();

    let mut prefix = VecDeque::new();
    let mut count = 0;
    for &b in word {
        if is_vowel(b) {
            if let Some(val) = prefix.back_mut() && *val != 0 {
                *val += 1;
            } else {
                prefix.push_back(1);
            }
        } else {
            prefix.push_back(0);
        }
    }

    let mut vowels = HashMap::with_capacity(5);
    let mut consonants = 0;
    let mut l = 0;
    let mut res = 0;
    for &b in word {
        if is_vowel(b) {
            *vowels.entry(b).or_insert(0) += 1;
        } else {
            consonants += 1;
        }

        if let Some(front) = prefix.front_mut() {
            *front -= 1;
            if *front <= 0 {
                prefix.pop_front();
            }
        }

        while vowels.len() == 5 && consonants >= k {
            if vowels.len() == 5 && consonants == k {
                res += 1 + prefix.front().unwrap_or(&0);
            }

            if is_vowel(word[l]) {
                let val = vowels.get_mut(&word[l]).unwrap();
                *val -= 1;
                if *val == 0 {
                    vowels.remove(&word[l]);
                }
            } else {
                consonants -= 1;
            }

            l += 1;
        }
    }

    res
}

pub fn main() {
    let word = "aadieuoh".to_string();
    let k = 1;
    println!("{}", count_of_substrings(word, k));
}
