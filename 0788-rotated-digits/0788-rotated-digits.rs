impl Solution {
    pub fn rotated_digits(n: i32) -> i32
    {
        let mut count: i32 = 0;
        
        for num in 0..=n 
        {
            if num.to_string().chars().any(|c| c == '3' || c == '4' || c == '7') { continue; }
            let r_num = num.to_string().chars().map(|c| rot(c)).collect::<String>().parse::<i32>();
            
            if num != r_num.expect("NOT A NUM!")
            {
              count += 1;  
            }
        
        }
        count
    }
}

    pub fn rot(v: char) -> char
    {
        match v
        {
            '0' => '0',
            '1' => '1',
            '2' => '5',
            '5' => '2',
            '6' => '9',
            '8' => '8',
            '9' => '6',
            _ => panic!("Fail!"),
        }
    }