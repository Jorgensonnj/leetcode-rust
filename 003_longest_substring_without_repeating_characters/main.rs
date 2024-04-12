use std::collections::HashMap;
use std::time::Instant;

// problem
// https://leetcode.com/problems/longest-substring-without-repeating-characters/

// my original code
pub fn my_length_of_longest_substring(s: String) -> i32 {
    let mut queue = "".to_string();

    let mut hash: HashMap<char, i32> = HashMap::new();

    let mut length = 0;
    for (i, char) in s.chars().enumerate() {
        if !hash.contains_key(&char) {
            queue.push(char);
            hash.insert(char, i as i32 );
        } else {
            length = if queue.len() > length { queue.len() } else { length };
            queue.push(char);
            let index = queue.chars().position(|c| c == char).unwrap() + 1;
            let q = queue.split_off(index);
            for key in queue.chars() {
                if key != char {
                    hash.remove_entry(&key);
                };
            }
            queue = q;
        }
    };
    return if queue.len() > length { queue.len() as i32 } else { length as i32 }

}

// someone else's code
// https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/218627/Rust-4ms
pub fn other_length_of_longest_substring(s: String) -> i32 {
    let mut hash = HashMap::with_capacity(s.len());
    let mut max = 0;
    let mut start = 0;
    let mut end = 0;

    for (i, item) in s.chars().enumerate() {
        if let Some(j) = hash.get(&item) {
            //checks that current symbol is in the current window.
            if *j >= start {
                let curr = end - start;
                if max < curr {
                    max = curr;
                }
                //move window
                start = *j + 1;
            }
        }
        end += 1;
        hash.insert(item, i);
    }
    let curr = end - start;
    if max < curr {
        max = curr;
    }
    max as i32
}

fn main() {


    let tests = vec![
        "aab",
        " ",
        "abcabcbb",
        "bbbbbb",
        "pwwkew",
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let my_time = Instant::now();
        let my_result = my_length_of_longest_substring( test.to_string() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_length_of_longest_substring( test.to_string() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());
    }
}
