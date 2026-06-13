impl Solution {
    pub fn valid_palindrome(s: String) -> bool 
    {
        let s_bytes = s.as_bytes();
        
        let mut si = 0;
        let mut ei = s_bytes.len() as i32 - 1;
        
        while si < ei 
        {
            if s_bytes[si as usize] != s_bytes[ei as usize] 
            {

                return Self::is_palindrome(s_bytes, si + 1, ei) 
                    || Self::is_palindrome(s_bytes, si, ei - 1);
            }
            si += 1;
            ei -= 1;
        }

        true
    }
    
    pub fn is_palindrome(ss: &[u8], mut s_i: i32, mut e_i: i32) -> bool
    {
        while s_i < e_i
        {
            if ss[s_i as usize] != ss[e_i as usize] { return false; }
            s_i += 1;
            e_i -= 1;
            
        }
        true
    }
}