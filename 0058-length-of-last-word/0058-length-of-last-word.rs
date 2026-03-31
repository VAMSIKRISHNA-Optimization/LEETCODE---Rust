impl Solution {
    pub fn length_of_last_word(s: String) -> i32 
    {
        s.trim().split_whitespace().last().expect("FAIL").len() as i32
    }
}