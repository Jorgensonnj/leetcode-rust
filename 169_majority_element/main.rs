use std::time::Instant;
use std::collections::HashMap;

// problem
// https://leetcode.com/problems/majority-element/

// my original code
pub fn my_majority_element(nums: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32,i32> = HashMap::new();

    for element in nums.iter() {
        map.entry(*element).and_modify(|counter| *counter += 1).or_insert(1);
    }

    let value = map
        .iter()
        .max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

    return *value.0;
}

// https://leetcode.com/problems/majority-element/submissions/1233388992
// someone else's code
pub fn other_majority_element(nums: &Vec<i32>) -> i32 {
    let mut count: usize = 1;
    let mut candidate: i32 = nums[0];
    for num in nums.into_iter().skip(1) {
        if count == 0 {
            candidate = *num;
        }
        if *num == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    candidate
}

fn main() {

    let tests = vec![
        vec![3,2,3],
        vec![2,2,1,1,1,2,2],
        vec![1,2,2,1,1,1,2]
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let my_time = Instant::now();
        let my_result = my_majority_element( test );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_majority_element( test );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());
    }
}
