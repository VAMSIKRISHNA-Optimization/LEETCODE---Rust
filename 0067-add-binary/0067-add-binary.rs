impl Solution {
    pub fn add_binary(a: String, b: String) -> String 
    {
        let mut res = String::new();
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        
        let mut i = a_bytes.len() as i32 - 1;
        let mut j = b_bytes.len() as i32 - 1;
        let mut carry = 0;

        while i >= 0 || j >= 0 || carry > 0 
        {
            let mut sum = carry;

            if i >= 0 
            {
                sum += (a_bytes[i as usize] - b'0') as u32;
                i -= 1;
            }
            
            if j >= 0 
            {
                sum += (b_bytes[j as usize] - b'0') as u32;
                j -= 1;
            }
            res.push(if sum % 2 == 0 { '0' } else { '1' });
            
            carry = sum / 2;
        }

        res.chars().rev().collect()
    }

}