use std::time::Instant;
use std::collections::HashMap;

// problem
// https://leetcode.com/problems/valid-anagram/

// my original code
pub fn my_is_anagram(s: String, t: String) -> bool {
    let mut map_s: HashMap<char,i32> = HashMap::new();
    let mut map_t: HashMap<char,i32> = HashMap::new();

    for c in s.chars() {
        *map_s.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        *map_t.entry(c).or_insert(0) += 1;
    }

    map_s.eq(&map_t)
}

//https://leetcode.com/problems/valid-anagram/submissions/1230559781/
// someone else's code
pub fn other_is_anagram(s: String, t: String) -> bool {
    let mut map = HashMap::new();
    // alternative to and_modify chain
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
    map.into_values().all(|v| v == 0)
}

fn main() {

    let tests = vec![
        ("anagram", "nagaram"),
        ("cat", "rat")
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let my_time = Instant::now();
        let my_result = my_is_anagram( test.0.to_string(), test.1.to_string() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_is_anagram( test.0.to_string(), test.1.to_string() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());
    }
}
