use std::time::Instant;

// problem
// https://leetcode.com/problems/longest-palindromic-substring/

// my original code
pub fn my_longest_palindrome(s: String) -> String {
    if s.len() < 2 { return s; }
    let string: Vec<char> = s.chars().collect();

    let mut start_ptr = 0;
    let mut end_ptr = 0;

    for index in 0..string.len() {
        // odd
        let mut left = index;
        let mut right = index;
        while ( left > 0 || left == 0 ) && right < string.len() {
            if string[ left ] == string[ right ] {
                if right - left > end_ptr - start_ptr {
                    start_ptr = left;
                    end_ptr = right;
                }

                if left == 0 { break; }

                left -= 1;
                right += 1;
            } else {
                break;
            }
        }

        // even
        left = index;
        right = index + 1;
        while ( left > 0 || left == 0 ) && right < string.len() {
            if string[ left ] == string[ right ] {
                if right - left > end_ptr - start_ptr {
                    start_ptr = left;
                    end_ptr = right;
                }

                if left == 0 { break; }

                left -= 1;
                right += 1;
            } else {
                break;
            }
        }
    }

    return string[start_ptr..end_ptr + 1].iter().collect();
}

// someone else's code
// https://leetcode.com/problems/longest-palindromic-substring/discuss/1210849/Rust
pub fn other_longest_palindrome(s: String) -> String {
    fn is_palidrone(s: &[u8]) -> bool {
        // Iterate left to right along with iterating from right to left,
        // make sure each spot is the same.
        // Returns false once the left is not equal to the right
        s.iter().zip(s.iter().rev()).all(|(l, r)| l == r)
    }

    for size in (1..=s.len()).rev() {
        match s.as_bytes()
            .windows(size)
            .find(|substr| is_palidrone(substr)) {
            Some(pal) => return String::from_utf8(pal.to_vec()).unwrap(),
            None => continue,
        }
    }
    // No palidrone found
    String::from("")
}

fn main() {

    let tests = vec![
        "1211111".to_string(),
        "semjhjlritnjgapzrakcqahaqetwllrldktufvdgkfusniv".to_string(),
        "a".to_string(),
        "bb".to_string(),
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let my_time = Instant::now();
        let my_result = my_longest_palindrome( test.clone() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_longest_palindrome( test.clone() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());
    }
}

// worked, but it was not fast enough
//pub fn my_longest_palindrome(s: String) -> String {
//
//    let mut start_prt = 0;
//    let mut end_prt = 0;
//    let mut length = 0;
//    for i in 0..s.len() {
//        for j in i..s.len() {
//            let slice = &s[i..j+1];
//            if is_palindrome(slice) {
//                if slice.len() > length {
//                    length = slice.len();
//                    start_prt = i;
//                    end_prt = j + 1;
//                }
//            }
//        };
//    };
//
//
//    return s[start_prt..end_prt].to_string();
//}
//
//pub fn is_palindrome(s: &str) -> bool {
//    let string: Vec<_> = s.chars().collect();
//    let mut high = string.len();
//    if high == 0 {
//        return false;
//    };
//
//    let mut low = 0;
//    while low < high {
//        if string[low] != string[high - 1] {
//            return false;
//        };
//        low += 1;
//        high -= 1;
//    };
//
//    return true;
//}
