use std::time::Instant;
use std::collections::HashSet;

// problem
// https://leetcode.com/problems/number-of-islands

// my original code
pub fn my_num_islands(grid: Vec<Vec<char>>) -> i32 {

    let mut visited: HashSet<String> = HashSet::new();
    let mut count = 0;
    for i in 0..grid.len() as i32 {
        for j in 0..grid[0].len() as i32 {
            if depth_first(&grid, i, j, &mut visited) {
                //println!("y: {}, x: {}", i, j);
                count += 1;
            };
        }
    }

    return count;
}

pub fn depth_first(grid: &Vec<Vec<char>>, row: i32, column: i32, visited: &mut HashSet<String> ) -> bool {

    let row_in_bounds    = 0 <= row   && row     < grid.len() as i32;
    let column_in_bounds = 0 <=column && column  < grid[0].len() as i32;
    if !row_in_bounds || !column_in_bounds { return false; };

    if grid[row as usize][column as usize] == '0' { return false; };

    let position = format!("{},{}", row, column);

    if visited.contains( &position ) {
        return false;
    };

    visited.insert(position);

    depth_first(grid, row - 1, column,     visited);
    depth_first(grid, row,     column - 1, visited);
    depth_first(grid, row + 1, column,     visited);
    depth_first(grid, row,     column + 1, visited);

    return true;
}

//https://leetcode.com/problems/number-of-islands/solutions/2105704/rust-iterative-dfs-bfs-with-comments
// someone else's code
pub fn other_num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let mut stack = vec![];
    let (m, n) = (grid.len(), grid[0].len());

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' {
                stack.push((i, j));
                while let Some((r, c)) = stack.pop() {
                    if r < m && c < n && grid[r][c] == '1' {
                        grid[r][c] = '0';
                        for rc in [0, 1, 0, !0, 0].windows(2) {
                            stack.push((r.wrapping_add(rc[0]), c.wrapping_add(rc[1])));
                        }
                    }
                }
                count += 1;
            }
        }
    }
    count
}

fn main() {

    let tests = vec![
        vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','0','0','0']
        ],
        vec![
            vec!['1','1','0','0','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','1','0','0'],
            vec!['0','0','0','1','1']
        ]
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let my_time = Instant::now();
        let my_result = my_num_islands( test.to_vec() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_num_islands( test.to_vec() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());
    }
}
