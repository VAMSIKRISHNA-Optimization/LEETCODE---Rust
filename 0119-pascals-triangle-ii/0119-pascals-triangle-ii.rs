impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> 
    {
        let mut triangle: Vec<Vec<i32>> = Vec::with_capacity(row_index as usize);

        for i in 0..=row_index as usize 
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
        
        triangle[row_index as usize].clone()
    }
        

}