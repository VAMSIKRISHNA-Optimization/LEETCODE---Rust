use std::collections::HashSet;
impl Solution 
{
    pub fn contains_duplicate(nums: Vec<i32>) -> bool 
    {
        let mut uniques: HashSet<&i32> = HashSet::new();
        
        for val in &nums
        {
            if !uniques.insert(val) { return true; }
        }
        
        false
        
    }
}