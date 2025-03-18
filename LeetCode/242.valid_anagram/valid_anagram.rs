use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len()!=t.len(){
            return false
        }
        let mut s_set: HashMap<char, i32> = HashMap::new();
        for elem in s.chars(){
            s_set.entry(elem).and_modify(|counter| *counter += 1).or_insert(1);
        }
        for elem in t.chars(){ 
            s_set.entry(elem).and_modify(|counter| *counter -= 1).or_insert(-1);
        }
        if s_set.values().map(|&x: &i32| x.abs()).sum::<i32>()==0 {
            return true
        }
        false
    }
}
