impl Solution {
    pub fn roman_to_int(s: String) -> i32 
    {
        let mut final_val: i32 = 0;
        let mut last_val : i32 = 0;
        
        s
        .chars()
        .rev()
        .for_each(|c| 
        {
            let curr_val: i32 = roman_to_int_conv(c);
            
            if curr_val < last_val
            {
                final_val -= curr_val;
            }
            else
            {
                final_val += curr_val;
            }
            
            last_val = curr_val;
        });
        
        final_val
      
      
    }
}
    pub fn roman_to_int_conv(c: char) -> i32
    {
        match c
        {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _   => panic!("Failed to parse char!")
            
        }
    }