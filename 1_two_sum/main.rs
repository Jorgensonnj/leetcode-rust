use std::collections::HashMap;
use std::time::Instant;

// my original code
pub fn my_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut hashmap = HashMap::new();

    let mut results: Vec<i32> = Vec::new();
    let mut index = 0;
    for num in nums {

        if let Some(val) = hashmap.get(&(target - num)) {
            results.push(*val);
            results.push(index);
            return results;
        };

        hashmap.insert(num, index);

        index = index + 1;
    };

    return results;
}

// someone else's code
// https://leetcode.com/problems/two-sum/discuss/715951/Rust%3A-HashMap-solution
pub fn other_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (index, num) in nums.iter().enumerate(){
        match map.get(&(target - *num)){
            Some(&index_two) => return vec![index_two, index as i32],
            None => map.insert(*num, index as i32),
        };
    }
    vec![]
}

fn main() {

    let my_time = Instant::now();
    let my_result = my_two_sum( vec![1,2,3,4,5], 9 );
    println!("Result: {:?}, Time: {:?}", my_result, my_time.elapsed());

    let other_time = Instant::now();
    let other_result = other_two_sum( vec![1,2,3,4,5], 9 );
    println!("Result: {:?}, Time: {:?}", other_result, other_time.elapsed());
}
