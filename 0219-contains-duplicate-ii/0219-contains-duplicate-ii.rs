use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool 
    {
        let mut unique_indices: HashMap<i32, Vec<usize>> = HashMap::with_capacity(nums.len());
        
        for (ind, val) in nums.iter().enumerate()
        {
            unique_indices.entry(*val).or_default().push(ind);
        }
        
        unique_indices
        .into_values()
        .filter(|indices| indices.len() > 1)
        .any(|indices|
        {
            indices.windows(2).any(|w| (w[1] - w[0]) <= k as usize)
        })
        
        
    }
}