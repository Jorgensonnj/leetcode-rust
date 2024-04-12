use std::time::Instant;

// problem
// https://leetcode.com/problems/rotate-array/

// my original code
pub fn my_rotate(nums: &mut Vec<i32>, k: i32) {
    let x = k as usize % nums.len();
    nums.rotate_right(x);
}

//https://leetcode.com/problems/rotate-array/submissions/1230454525/
// someone else's code
pub fn other_rotate(nums: &mut Vec<i32>, k: i32) {
    if nums.len() <= 1 {
        return;
    }

    let size = nums.len();
    let k = (k as usize) % size;

    if k == 0 {
        return;
    }

    other_reverse(nums, 0, size - 1);

    other_reverse(nums, 0, k - 1);
    other_reverse(nums, k, size - 1);
}

fn other_reverse(nums: &mut Vec<i32>, mut start: usize, mut end: usize) {
    while start < end {
        nums.swap(start, end);
        start += 1;
        end -= 1;
    }
}

fn main() {

    let tests = vec![
        (vec![1,2,3,4,5,6,7], 3),
        (vec![1], 2)
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let (mut my_vec, my_k) = (test.0.clone(), test.1);
        let (mut other_vec, other_k) = (test.0.clone(), test.1);

        let my_time = Instant::now();
        my_rotate( &mut my_vec, my_k );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_vec, my_time.elapsed());

        let other_time = Instant::now();
        other_rotate( &mut other_vec, other_k );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_vec, other_time.elapsed());
    }
}
