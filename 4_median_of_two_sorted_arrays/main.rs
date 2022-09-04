use std::time::Instant;

// problem
// https://leetcode.com/problems/median-of-two-sorted-arrays/

// my original code
pub fn my_find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    /* the first version of my code
    *  let l1 = nums1.len();
    *  let l2 = nums2.len();
    *  let denominator_total = l1 + l2;

    *  let sum1: i32 = nums1.iter().sum();
    *  let sum2: i32 = nums2.iter().sum();
    *  let numerator_total = sum1 + sum2;

    *  (numerator_total / denominator_total as i32) as f64
    */

    let mut small = nums1;
    let mut big   = nums2;

    if small.len() > big.len() {
        (big, small) = (small, big);
    }

    let small_l = small.len();
    let big_l   = big.len();

    let total = small_l + big_l;
    let half  = total / 2;

    let mut l = 0;
    let mut r = small_l;

    loop {
        let small_mid = (l + r) / 2;
        let big_mid = half - small_mid;

        let small_left = if small_mid != 0 {
            match small.get(small_mid - 1) {
                Some(v) if  v >= &0 => f64::from(*v),
                _ => f64::NEG_INFINITY,

            }
        } else { f64::NEG_INFINITY };

        let small_right = if small_mid != small_l {
            match small.get(small_mid) {
                Some(v) if (small_mid) < small_l => f64::from(*v),
                _ => f64::INFINITY,

            }
        } else { f64::INFINITY };

        let big_left = if big_mid != 0 {
            match big.get(big_mid - 1) {
                Some(v) if  v >= &0 => f64::from(*v),
                _ => f64::NEG_INFINITY,

            }
        } else { f64::NEG_INFINITY };

        let big_right = if big_mid != big_l {
            match big.get(big_mid) {
                Some(v) if (big_mid) < big_l => f64::from(*v),
                _ => f64::INFINITY,

            }
        } else { f64::INFINITY };

        if small_left <= big_right && big_left <= small_right {
            if total % 2 == 1 {
                return small_right.min(big_right);
            } else {
                return (small_right.min(big_right) + small_left.max(big_left)) / 2.0;
            }
        } else if small_left > big_right {
            r = small_mid - 1;
        } else {
            l = small_mid + 1;
        }
    };
}

// someone else's code
// Unknown source link
pub fn other_find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total = nums1.len() + nums2.len();
    let mut half = (total / 2) + 1;
    let even = total % 2 == 0;
    if even {
        half -= 1;
    }
    let mut count = 0;
    let mut x1 = 0;
    let mut x2 = 0;
    let mut previous = 0;
    let mut current = 0;
    if nums1.is_empty() && nums2.len() == 1 {
        return nums2[0] as f64;
    } else if nums2.is_empty() && nums1.len() == 1 {
        return nums1[0] as f64;
    }
    while count <= half {
        previous = current;
        if x1 == nums1.len() {
            current = nums2[x2];
            x2 += 1;
        } else if x2 == nums2.len() {
            current = nums1[x1];
            x1 += 1;
        } else if nums1[x1] < nums2[x2] {
            current = nums1[x1];
            x1 += 1;
        } else {
            current = nums2[x2];
            x2 += 1;
        }
        count += 1;
    }
    if even {
        (current as f64 + previous as f64) / 2 as f64
    } else {
        previous as f64
    }
}

fn main() {

    //let (vec1, vec2) = (vec![1,3], vec![2]);
    //let vec3 = vec1.clone();
    //let vec4 = vec2.clone();

    let (vec1, vec2) = (vec![1,2] , vec![3,4]);
    let vec3 = vec1.clone();
    let vec4 = vec2.clone();

    let my_time = Instant::now();
    let my_result = my_find_median_sorted_arrays( vec1, vec2 );
    println!("Result: {:?}, Time: {:?}", my_result, my_time.elapsed());

    let other_time = Instant::now();
    let other_result = other_find_median_sorted_arrays( vec3, vec4 );
    println!("Result: {:?}, Time: {:?}", other_result, other_time.elapsed());
}
