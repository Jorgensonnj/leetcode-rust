use std::time::Instant;

// problem
// https://leetcode.com/problems/longest-palindromic-substring/

// my original code
pub fn my_conversion(s: String, num_rows: i32) -> String {

    let mut new_string: Vec<char> = vec![];
    let org_string: Vec<char> = s.chars().collect();

    let len = org_string.len();

    let mut jump = (( num_rows * 2 ) - 2) as usize;
    if jump == 0 { jump = 1 };

    let mut even_toggle = 0;
    let mut odd_toggle = jump;

    for row in 0..num_rows {
        let mut index = row as usize;
        let mut num_seen = 0;
        while index < len {
            new_string.push( org_string[ index ] );

            num_seen += 1;

            if row == 0 || row == num_rows - 1 {
                index += jump;
            } else {
                if num_seen % 2 == 0 {
                    index += even_toggle;
                } else {
                    index += odd_toggle;
                }
            }
        };

        even_toggle += 2;
        if odd_toggle > 0 {
            odd_toggle -= 2;
        }
    }

    return new_string.iter().collect();
}

//https://leetcode.com/problems/zigzag-conversion/submissions/825989229/
// someone else's code
pub fn other_conversion<S>(s: S, num_rows: i32) -> String where S: AsRef<str> {
    let input_string = s.as_ref();
    let input_bytes = input_string.as_bytes();

    // The zigzag encoded string will have the same length as the input string.
    let mut result = String::with_capacity(input_string.len());

    // One logical block is a segment consisting of the initial descender plus the rise up.
    let rows = num_rows;
    let characters_per_block = std::cmp::max(num_rows + (num_rows - 2), 1);
    let blocks = (input_string.len().saturating_sub(1) as i32 / characters_per_block) + 1;

    for row in 0 .. rows {
        for block in 0 .. blocks {
            // Index of the descending character in the current block.
            let d_index = (block * characters_per_block) + row;
            if d_index < input_bytes.len() as i32 {
                result.push(input_bytes[d_index as usize] as char);
            }

            // Index of the ascending character in the current block.
            if row > 0 && row < (rows - 1) {
                let index_base = (block * characters_per_block) + rows;
                let height = (rows - 2) - row;
                let a_index = index_base + height;
                if a_index < input_bytes.len() as i32 {
                    result.push(input_bytes[a_index as usize] as char);
                }
            }
        }
    }

    result
}

fn main() {

    // test 1
    //let s1 = "PAYPALISHIRING".to_string();
    //let s2 = s1.clone();
    //let num = 4;

    // test 2
    //let s1 = "PAYPALISHIRING".to_string();
    //let num = 3;
    //let s2 = s1.clone();

    // test 3
    let s1 = "PAYPALISHIRING".to_string();
    let s2 = s1.clone();
    let num = 6;

    // test 3
    //let s1 = "bb".to_string();
    //let s2 = s1.clone();

    let my_time = Instant::now();
    let my_result = my_conversion( s1, num );
    println!("Result: {:?}, Time: {:?}", my_result, my_time.elapsed());

    let other_time = Instant::now();
    let other_result = other_conversion( s2, num );
    println!("Result: {:?}, Time: {:?}", other_result, other_time.elapsed());
}
