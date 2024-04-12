use std::time::Instant;
use std::string::String;

// problem
// https://leetcode.com/problems/is-subsequence/

// my original code
pub fn my_is_subsequence(s: String, t: String) -> bool {

    let (mut pointer_s, mut pointer_t): (usize, usize) = (0, 0);
    let s_vec: Vec<char> = s.chars().collect();
    let t_vec: Vec<char> = t.chars().collect();

    while pointer_s < s.len() && pointer_t < t.len() {
        if s_vec[pointer_s] == t_vec[pointer_t] {
            pointer_s += 1;
        }
        pointer_t += 1;
    }

    return pointer_s == s.len();
}

//https://leetcode.com/problems/is-subsequence/submissions/1230565625/
// someone else's code
pub fn other_is_subsequence(s: String, t: String) -> bool {

    if s=="" {
        return true;
    }

    let mut ss = s.chars().into_iter();
    let mut cc = ss.next();
    for tc in t.chars() {
        match cc {
            Some(ccc) => {
                if tc == ccc {
                    cc = ss.next();
                }
            }
            None => return true
        }
    }
    match cc {
        Some(_) => return false,
        None => return true
    }
}

fn main() {

    let tests = vec![
        ("abc", "ahbgdc"),
        ("axc", "ahbgdc"),
        ("aaaaa", "bbaaaaa")
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let my_time = Instant::now();
        let my_result = my_is_subsequence( test.0.to_string(), test.1.to_string() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_is_subsequence( test.0.to_string(), test.1.to_string() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());
    }
}
