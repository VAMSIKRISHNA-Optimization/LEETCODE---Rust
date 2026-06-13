impl Solution {
    pub fn rotate_string(mut s: String, goal: String) -> bool 
    {
        if s.len() != goal.len() { return false; }
        if s == goal { return true; }
        
        let mut rot = 0;
        
        while rot < s.len()
        {
            s = Self::rotate_left_one(&s); 
            if s == goal { return true; }
            rot += 1;
        }
        
        false
    }
    
    fn rotate_left_one(s: &str) -> String 
    {
        if s.is_empty() 
        {
            return s.to_string();
        }
        
        let (first, rest) = s.split_at(1);
        format!("{}{}", rest, first)
    }
}