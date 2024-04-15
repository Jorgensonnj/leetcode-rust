use std::time::Instant;

// problem
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

// my original code
pub fn my_remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    return nums.len() as i32;
}

// https://leetcode.com/problems/remove-duplicates-from-sorted-array/submissions/1232693649
// someone else's code
pub fn other_remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let nums2 = nums.clone();
    let mut recorder: i32 = 0;
    nums.clear();

    for val in nums2.into_iter() {
        if nums.len() == 0 {
            nums.push(val);
            recorder = val;
        } else {
            match recorder != val {
                true => {
                    nums.push(val);
                    recorder = val;
                },
                _ => {
                    continue;
                },
            }
        }
    }

    nums.len() as i32
}

fn main() {

    let tests = vec![
        vec![1,1,2],
        vec![0,0,1,1,1,2,2,3,3,4],
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let mut my_vec = test.clone();
        let mut other_vec = test.clone();

        let my_time = Instant::now();
        let my_result = my_remove_duplicates( &mut my_vec );
        println!("Test: {}, Result: Length = {:?} Array = {:?}, Time: {:?}", index, my_result, my_vec, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_remove_duplicates( &mut other_vec );
        println!("Test: {}, Result: Length = {:?} Array = {:?}, Time: {:?}", index, other_result, other_vec, other_time.elapsed());
    }
}
