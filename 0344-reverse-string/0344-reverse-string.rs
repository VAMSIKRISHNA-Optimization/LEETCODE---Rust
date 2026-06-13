impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) 
    {
        if s.is_empty() { return; }
        let mut start_ind = 0;
        let mut end_ind   = s.len() - 1;
        
        while start_ind < end_ind
        {
            s.swap(start_ind, end_ind);
            start_ind   += 1;
            end_ind     -= 1;
        }
        
    }
}