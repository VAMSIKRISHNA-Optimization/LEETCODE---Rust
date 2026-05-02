impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> 
    {
        let pairs = get_divisor_pairs(area);
        
        pairs
        .into_iter()
        .map(|(a, b)| if a < b { (b, a) } else { (a, b) })
        .min_by_key(|(a, b)| a - b)
        .map(|(l, w)| vec![l, w])
        .unwrap()
    }
}

    
    pub fn get_divisor_pairs(num: i32) -> Vec<(i32, i32)> 
    {
        if num < 1 { return vec![]; }
        
        let mut pairs = Vec::new();
        let sqrt_n = (num as f64).sqrt() as i32;
    
        for i in 1..=sqrt_n 
        {
            if num % i == 0 
            {
                pairs.push((i, num / i));
            }
        }
    
        pairs
    }




