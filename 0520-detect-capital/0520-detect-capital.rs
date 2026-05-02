impl Solution {
    pub fn detect_capital_use(word: String) -> bool 
    {
        let count = word.chars().filter(|c| c.is_uppercase()).count();
        let n = word.len();

        if count == n { return true; }
        if count == 0 { return true; }

        if count == 1 && word.chars().next().unwrap().is_uppercase() 
        {
            return true;
        }

        false
    }
}
