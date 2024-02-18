use std::time::Instant;

// problem
// https://leetcode.com/problems/string-to-integer-atoi/

// my original code
pub fn my_atoi(s: String) -> i32 {
    let mut is_num = false;
    let mut result: Vec<char> = Vec::new();
    let mut is_negative = false;

    let mut iterator = s.chars();
    while let Some(character) = iterator.next() {
        match character {
            character if character.is_digit(10)      => { is_num = true; result.push(character); },
            character if character == '-' &&  is_num => { break; },
            character if character == '-' && !is_num => { is_num = true; is_negative = true;  },
            character if character == '+' &&  is_num => { break; },
            character if character == '+' && !is_num => { is_num = true; },
            character if character == ' ' &&  is_num => { break; },
            _                                        => { break; }
        }
    }

    let mut new_result: String = result.into_iter().collect();
    if is_negative {
        new_result = String::from("-") + &new_result;
    }

    let val = match new_result.parse::<i32>() {
        Ok(val) => val,
        Err(message) => match message.kind(){
            std::num::IntErrorKind::PosOverflow => i32::MAX,
            std::num::IntErrorKind::NegOverflow => i32::MIN,
            _ => 0
        }
    };

    return val;
}


// someone else's code
// https://leetcode.com/problems/string-to-integer-atoi/solutions/2981953/0ms-extremly-readable-solution/
pub fn other_atoi1(s: String) -> i32 {
     let mut read = s.chars();
     let mut result: String = String::new();
     let mut reading_strings = false;
     let mut negative = false;

     while let Some(thing) = read.next(){
         match thing {
             '0'..='9' => { reading_strings = true; result.push(thing); },
             '-' => { if reading_strings { break; } reading_strings = true; negative = true; },
             '+' => { if reading_strings { break; } reading_strings = true; },
             ' ' => if reading_strings { break; },
             _ => break
         }
     }

     if negative { result = String::from("-") + &result; }

     match result.parse::<i32>() {
         Ok(val) => val,
         Err(message) => match message.kind(){
             std::num::IntErrorKind::PosOverflow => i32::MAX,
             std::num::IntErrorKind::NegOverflow => i32::MIN,
             _ => 0
         }
     }
}

// someone else's code
// https://leetcode.com/problems/string-to-integer-atoi/solutions/2207474/rust-0ms-simple-compact-functional/
pub fn other_atoi2(s: String) -> i32 {
    let s = s.trim_start();
    let (s, sign) = match s.strip_prefix('-') {
        Some(s) => (s, -1),
        None => (s.strip_prefix('+').unwrap_or(s), 1),
    };
    s.chars()
        .map(|c| c.to_digit(10))
        .take_while(Option::is_some)
        .flatten()
        .fold(0, |acc, digit| {
            acc.saturating_mul(10).saturating_add(sign * digit as i32)
        })
}

fn main() {

    let tests = vec![
        "-00123".to_string(),
        "00123".to_string(),
        " some words -4193 with words".to_string(),
        "-4193 with words".to_string(),
        "-919838338482".to_string(),
        "+1".to_string(),
        "+-12".to_string()
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let my_time = Instant::now();
        let my_result = my_atoi( test.clone() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other1_time = Instant::now();
        let other1_result = other_atoi1( test.clone() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other1_result, other1_time.elapsed());

        let other2_time = Instant::now();
        let other2_result = other_atoi2( test.clone() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other2_result, other2_time.elapsed());
    }

}
