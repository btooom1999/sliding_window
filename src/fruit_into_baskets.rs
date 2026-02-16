use std::collections::HashMap;

fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::new();
    let mut l = 0;
    let mut res = 0;
    for (r, &num) in fruits.iter().enumerate() {
        *hashmap.entry(num).or_insert(0) += 1;
        while hashmap.len() > 2 {
            if let Some(val) = hashmap.get_mut(&fruits[l]) {
                *val -= 1;
                if *val == 0 {
                    hashmap.remove(&fruits[l]);
                }
            }

            l += 1;
        }

        res = res.max(r - l);
    }

    res as i32
}

pub fn main() {
    let fruits = [0,1,2,2].to_vec();
    println!("{}", total_fruit(fruits));
}
