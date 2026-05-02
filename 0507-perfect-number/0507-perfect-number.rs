impl Solution {
    pub fn check_perfect_number(num: i32) -> bool 
    {
        if num <= 1 { return false; }

        let mut sum = 0;
        let sqrt_n = (num as f64).sqrt() as i32;

        for i in 1..=sqrt_n 
        {
            if num % i == 0 
            {
                sum += i; // Add the divisor
                
                let pair = num / i;
                // Add the partner if it's not the same and not the number itself
                if pair != i && pair != num 
                {
                    sum += pair;
                }
            }
        }

        sum == num
    }
}



