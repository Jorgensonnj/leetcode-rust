use std::time::Instant;

// problem
// https://leetcode.com/problems/happy-number/

// my original code
pub fn my_is_happy(mut n: i32) -> bool {

    let mut stack: Vec<i32> = vec![];
    loop {
        if n == 1 { return true }
        let mut i = 0;
        while n > 0 {
            i = i + (n % 10).pow(2);
            n = n / 10;
        }
        if stack.contains(&i) { return false }
        stack.push(i);
        n = i;
    }

}

//https://leetcode.com/problems/happy-number/submissions/1203932039/
// someone else's code
pub fn other_is_happy(n: i32) -> bool {
        fn digit_square_sum(num: &i32) -> i32 {
            let mut num = *num;
            let mut ret = 0;
            while num > 0 {
                ret += (num % 10).pow(2);
                num /= 10;
            }

            ret
        }

        let mut num = n;
        let mut seen: std::collections::HashSet<i32> = std::collections::HashSet::new();
        seen.insert(num);

        loop {
            num = digit_square_sum(&num);
            if num == 1 {
                return true;
            } else if seen.contains(&num) {
                return false;
            }

            seen.insert(num);
        }
}

fn main() {

    let tests = vec![19, 2, 7];

    for ( index, test ) in tests.iter().enumerate() {

        let my_time = Instant::now();
        let my_result = my_is_happy( *test );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_is_happy( *test );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());
    }
}
