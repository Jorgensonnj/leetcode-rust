use std::time::Instant;

// problem
// https://leetcode.com/problems/reverse-integer/

// my original code
pub fn my_reverse(x: i32) -> i32 {
    let signed = if x < 0 { -1 } else { 1 };
    return recusion(x.abs(), 0) * signed;
}

pub fn recusion(x: i32, reverse: i32) -> i32 {
    match x {
        x if x < 0 => 0,
        x if x == 0 => reverse,
        _ => {
            recusion( x / 10, match reverse.checked_mul(10) {
                None      => 0,
                Some(sum) => match sum.checked_add( x % 10 ) {
                    None      => 0,
                    Some(sum) => sum
                }
            })
        }
    }
}

// someone else's code
pub fn other_reverse(x: i32) -> i32 {
    let mut res: i32 = 0;
    let mut cur: i32 = x;
    while cur != 0 {
        match res.checked_mul(10) {
            None => return 0,
            Some(tmp) => match tmp.checked_add(cur % 10) {
                None => return 0,
                Some(fine) => {
                    res = fine;
                }
            }
        }
        cur = cur / 10;
    }
    res
}

fn main() {

    let tests = vec![
        123,
        -123,
        120,
        1534236469,
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let my_time = Instant::now();
        let my_result = my_reverse( test.clone() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_reverse( test.clone() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());
    }
}
