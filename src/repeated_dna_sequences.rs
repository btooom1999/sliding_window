use std::collections::{HashSet, VecDeque};

fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let mut hashset = HashSet::new();
    let mut window = VecDeque::new();

    let mut res = HashSet::new();
    let mut i = 0;
    let s = s.as_bytes();
    for (j, &c) in s.iter().enumerate() {
        window.push_back(c);

        if j > 9 {
            window.pop_front();
        }

        if j >= 9 {
            if hashset.contains(&window.clone()) {
                res.insert(String::from_utf8(window.clone().into()).unwrap());
            }

            hashset.insert(window.clone());
        }
    }

    res.into_iter().collect::<Vec<_>>()
}

pub fn main() {

}
