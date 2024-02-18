use std::time::Instant;

// problem
// https://leetcode.com/problems/merge-sorted-array/description

// my original code
pub fn my_merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

    let (mut m, mut n) = (m as usize, n as usize);
    while n > 0 {
        if m > 0 && nums1[m - 1] > nums2[n - 1] {
            nums1[m + n - 1] = nums1[m - 1];
            m -= 1;
        } else {
            nums1[m + n - 1] = nums2[n - 1];
            n -= 1;
        }
    }
}

//https://leetcode.com/problems/merge-sorted-array/solutions/1486149/idiomatic-rust-0ms-with-matching-and-std-cmp-ordering/
// someone else's code
pub fn other_merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    use std::cmp::Ordering::{Less, Equal, Greater};

    let (mut i1, mut i2) = (m - 1, n - 1);

    for i in (0..(m+n) as usize).rev() {
        match nums1.get(i1 as usize).cmp(&nums2.get(i2 as usize)) {
            Less            => { nums1[i] = nums2[i2 as usize]; i2 -= 1; },
            Equal | Greater => { nums1[i] = nums1[i1 as usize]; i1 -= 1; },
        };
    }
}

fn main() {

    let tests = vec![
        (vec!(1,2,3,0,0,0), 3, vec!(2,5,6), 3),
        (vec!(1), 1, vec!(), 0),
        (vec!(0), 0, vec!(1), 1),
    ];

    for ( index, test ) in tests.iter().enumerate() {
        let mut my_result    = test.0.clone();
        let mut other_result = test.0.clone();

        let my_time = Instant::now();
        my_merge( &mut my_result, test.1, &mut test.2.clone(), test.3 );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        other_merge( &mut other_result, test.1, &mut test.2.clone(), test.3 );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());
    }
}
