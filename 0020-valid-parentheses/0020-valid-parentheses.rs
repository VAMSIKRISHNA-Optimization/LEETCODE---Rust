impl Solution {
    pub fn is_valid(s: String) -> bool 
    {
        let mut pars: Vec<char> = Vec::new();
    
        for c in s.chars() 
        {
            match c 
            {
                '(' | '{' | '[' => pars.push(c),
                ')' => if pars.pop() != Some('(') { return false; },
                '}' => if pars.pop() != Some('{') { return false; },
                ']' => if pars.pop() != Some('[') { return false; },
                _ => panic!("Fail!"),
            }
        }
    
        pars.is_empty()
    }
}