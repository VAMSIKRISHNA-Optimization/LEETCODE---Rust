use std::collections::HashSet;
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> 
    {
        let length = nums.len() as i32; 
        let num_set: HashSet<i32> = nums.into_iter().collect();
        (1..=length).filter(|n| !num_set.contains(n)).collect::<Vec<i32>>()
    }
}