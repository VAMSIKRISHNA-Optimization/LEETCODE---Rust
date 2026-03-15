impl Solution 
{
    pub fn is_palindrome(mut x: i32) -> bool 
    {
        if x < 0 { return false; }
        if x % 10 == 0 && x!= 0 { return false; }

        let mut rev_x: i32 = 0;

        while x > rev_x 
        {
            rev_x = rev_x * 10 + x % 10;
            x /= 10;
        }

        x == rev_x || x == rev_x/10

        
    }
}