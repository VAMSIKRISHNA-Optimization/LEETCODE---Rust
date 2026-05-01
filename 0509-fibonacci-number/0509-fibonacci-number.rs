impl Solution {
    pub fn fib(n: i32) -> i32
    {
        
        if n == 0 { return 0; }
        if n == 1 { return 1; }
        
        let mut last_2: i32 = 0;
        let mut last_1: i32 = 1;
        
        let mut num: i32 = 2;
        let mut next_fib_num: i32 = 0;
        
        while num <= n
        {
            next_fib_num = last_2 + last_1;
            last_2 = last_1;
            last_1 = next_fib_num;
            num += 1;
        }
        
        next_fib_num
        
    }
   
}