impl Solution 
{
    pub fn add_strings(num1: String, num2: String) -> String 
    {
        let s1 = num1.as_bytes();
        let s2 = num2.as_bytes();
        
        let mut i = s1.len() as i32 - 1;
        let mut j = s2.len() as i32 - 1;
        
        let mut carry  = 0;
        let mut result = String::new();

        while i >= 0 || j >= 0 || carry > 0 
        {
            let mut sum = carry;

            if i >= 0 
            {
                sum += (s1[i as usize] - b'0') as i32;
                i -= 1;
            }

            if j >= 0 
            {
                sum += (s2[j as usize] - b'0') as i32;
                j -= 1;
            }

            carry = sum / 10;
            let digit = (sum % 10) as u8 + b'0';

            result.push(digit as char);
        }

        result.chars().rev().collect()
    }
}
