use std::time::Instant;

// problem
// https://leetcode.com/problems/longest-common-prefix/

// my original code
pub fn my_longest_common_prefix(mut strs: Vec<String>) -> String {

    while strs.len() > 1 {
        let first  = strs.pop().unwrap();
        let second = strs.pop().unwrap();
        strs.push(
            first.chars()
                .zip(second.chars())
                .take_while(|(a,c)| a == c)
                .map(|(c,_)|c)
                .collect()
            )
    }

    return strs[0].clone();
}

//https://leetcode.com/problems/longest-common-prefix/solutions/3244957/idiomatic-and-elegant-rust-solution/
// someone else's code
pub fn other_longest_common_prefix(input: Vec<String>) -> String {
    input.into_iter().reduce(|acc,cur|{
        acc.chars()
           .zip(cur.chars())
           .take_while(|(a,c)| a == c)
           .map(|(c,_)|c)
           .collect()
    }).unwrap()
}

fn main() {

    let tests = vec![
        vec![
            "flow".to_string(),
            "flight".to_string(),
            "flip".to_string()
        ],
        vec![
            "nicholas".to_string(),
            "nicholes".to_string(),
            "nichlas".to_string()
        ]
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let my_time = Instant::now();
        let my_result = my_longest_common_prefix( test.to_vec() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_longest_common_prefix( test.to_vec() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());
    }
}
