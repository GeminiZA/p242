struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map1: HashMap<char, u16> = HashMap::new();
        let mut map2: HashMap<char, u16> = HashMap::new();
        for c in s.chars() {
            map1.entry(c).and_modify(|value| *value += 1).or_insert(1);
        }
        for c in t.chars() {
            map2.entry(c).and_modify(|value| *value += 1).or_insert(1);
        }
        if map1 == map2 {
            return true;
        } else {
            return false;
        }
    }
}
fn main() {
    let s = String::from("rat");
    let t = String::from("car");
    let result = Solution::is_anagram(s, t);
    println!("{}", result);
}

