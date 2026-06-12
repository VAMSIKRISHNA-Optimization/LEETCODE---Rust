impl Solution {
    pub fn is_palindrome(s: String) -> bool 
    {
        if s.is_empty() { return true; }
        
        let clean_string: String = s.chars()
                                    .filter(|c| c.is_ascii_alphanumeric())
                                    .map(|c| c.to_ascii_lowercase())
                                    .collect();
        
        clean_string.chars().eq(clean_string.chars().rev())
    }
}