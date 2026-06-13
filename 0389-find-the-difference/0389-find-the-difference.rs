impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char 
    {
        if s.is_empty() && t.len() == 1 { return t.chars().next().unwrap(); };
        if t.is_empty() && s.len() == 1 { return s.chars().next().unwrap(); };
        
        (t.bytes().map(|b| b as u32).sum::<u32>() - s.bytes().map(|b| b as u32).sum::<u32>()) as u8 as char
    }
}