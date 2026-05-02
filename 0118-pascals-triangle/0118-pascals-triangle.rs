impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> 
    {
        let mut triangle: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);

        for i in 0..num_rows as usize 
        {
            let mut row = vec![1; i + 1];
            
            if i > 1
            {
                let prev_row = &triangle[i - 1];
                for (j, pair) in prev_row.windows(2).enumerate() 
                {
                    row[j + 1] = pair[0] + pair[1];
                }
            }
            
            triangle.push(row);
        }
        
        triangle
    }

}