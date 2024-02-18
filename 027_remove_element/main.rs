use std::time::Instant;

// problem
// https://leetcode.com/problems/remove-element/

// my original code,
pub fn my_remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {

    let temp: Vec<i32> = nums.iter()
        .filter(|&x| x.ne(&val))
        .cloned()
        .collect();

    *nums = temp;
    nums.iter().count() as i32
}

//https://leetcode.com/problems/merge-sorted-array/solutions/1486149/idiomatic-rust-0ms-with-matching-and-std-cmp-ordering/
// someone else's code
pub fn other_remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut ins_idx = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[ins_idx] = nums[i];
            ins_idx += 1;
        }
    }
    ins_idx as i32
}

fn main() {

    let tests = vec![
        (vec!(1,2,3,0,0,0), 3),
        (vec!(1), 1),
        (vec!(0), 0),
    ];

    for ( index, test ) in tests.iter().enumerate() {
        let my_time = Instant::now();
        let my_result = my_remove_element( &mut test.0.clone(), test.1 );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_remove_element( &mut test.0.clone(), test.1 );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());
    }
}
