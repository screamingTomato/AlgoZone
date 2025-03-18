use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut maps:HashMap<i32, usize> = HashMap::new();
        for (index, elem) in nums.iter().enumerate(){
            maps.entry(*elem).or_insert(index);
            let index2 = *maps.get(&(target-*elem)).unwrap_or(&index);
            if index!=index2{
                return vec![index as i32, index2 as i32]
            }
        };
        vec![0,0];
    }
}
