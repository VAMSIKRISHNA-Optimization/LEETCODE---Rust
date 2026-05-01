impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 
    {
        let mut first_max:  Option<i32> = None;
        let mut second_max: Option<i32> = None;
        let mut third_max:  Option<i32> = None;

        for num in nums 
        {
            if Some(num) == first_max || Some(num) == second_max || Some(num) == third_max 
            {
                continue;
            }

            if first_max.is_none() || num > first_max.unwrap() 
            {
                third_max = second_max;
                second_max = first_max;
                first_max = Some(num);
            } 
            else if second_max.is_none() || num > second_max.unwrap() 
            {
                third_max = second_max;
                second_max = Some(num);
            } 
            else if third_max.is_none() || num > third_max.unwrap() 
            {
                third_max = Some(num);
            }
        }

        third_max.unwrap_or_else(|| first_max.unwrap())
    }
}