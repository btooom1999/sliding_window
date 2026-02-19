fn longest_beautiful_substring(word: String) -> i32 {
    let word = word.as_bytes();
    let mut l = 0;
    let mut res = 0;
    let mut count = 1;
    for r in 1..word.len() {
        if word[r] < word[r-1] {
            l = r;
            count = 1;
        } else if word[r] > word[r-1] {
            count += 1;
        }

        if count == 5 {
            res = res.max(r - l + 1);
        }
    }

    res as _
}

pub fn main() {
    let word = "aeiaaioaaaaeiiiiouuuooaauuaeiu".to_string();
    println!("{}", longest_beautiful_substring(word));
}
