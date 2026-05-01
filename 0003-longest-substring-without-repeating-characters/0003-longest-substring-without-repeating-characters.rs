impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 
    {
        let mut checker = String::new();
        let mut scores  = Vec::<i32>::new();
        
        let mut left = 0;
        let s_chars: Vec<char> = s.chars().collect();

        for c in s.chars() 
        {
            if checker.is_empty() 
            {
                checker.push(c);
            } else {
                if checker.contains(c) 
                {
                    scores.push(checker.len() as i32);
                    
                    while checker.contains(c) 
                    {
                        checker.remove(0);
                        left += 1;
                    }
                    checker.push(c);
                } else {
                    checker.push(c);
                }
            }
        }
        
        scores.push(checker.len() as i32);

        scores.iter().max().copied().unwrap_or(0)
    }

}