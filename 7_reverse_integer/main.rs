use std::time::Instant;

// problem
// https://leetcode.com/problems/reverse-integer/

// my original code
pub fn my_reverse(x: i32) -> i32 {
    let abs = x.abs();
    let one = if x < 0 { -1 } else { 1 };
    return recusion(abs, 1) * one;
}

pub fn recusion(x: i32, factor: i32) -> i32 {
    if x == 0 {
        return 0;
    } else {
        let mod_num = x % 10;
        println!("mod_num: {}, x: {}, factor: {}", mod_num, x, factor);
        return mod_num + recusion(x / 10, factor * 10) * factor;
    }
}



// someone else's code
//pub fn other_reverse(x: i32) -> i32 {
//}

fn main() {

    // test 1
    let num1 = 123;
    //let num2 = 123;
    //let s2 = s1.clone();

    // test 2
    //let s1 = "semjhjlritnjgapzrakcqahaqetwllrldktufvdgkfusniv".to_string();
    //let s2 = s1.clone();

    // test 3
    //let s1 = "a".to_string();
    //let s2 = s1.clone();

    // test 3
    //let s1 = "bb".to_string();
    //let s2 = s1.clone();

    let my_time = Instant::now();
    let my_result = my_reverse( num1 );
    println!("Result: {:?}, Time: {:?}", my_result, my_time.elapsed());

    //let other_time = Instant::now();
    //let other_result = other_reverse( num );
    //println!("Result: {:?}, Time: {:?}", other_result, other_time.elapsed());
}
