use std::time::Instant;
use std::collections::HashMap;

// problem
// https://leetcode.com/problems/roman-to-integer/

// my original code
pub fn my_roman_to_int(s: String) -> i32 {
    let mut integer = 0;
    let mut roman_numeral_list = HashMap::new();
    roman_numeral_list.insert('I', 1);
    roman_numeral_list.insert('V', 5);
    roman_numeral_list.insert('X', 10);
    roman_numeral_list.insert('L', 50);
    roman_numeral_list.insert('C', 100);
    roman_numeral_list.insert('D', 500);
    roman_numeral_list.insert('M', 1000);

    let mut iterator = s.chars();
    let mut prev_char = iterator.next().unwrap();
    integer += roman_numeral_list.get(&prev_char).unwrap();
    for character in iterator {
        match character {
            'M' | 'D' if prev_char == 'C' => {
                integer += roman_numeral_list.get(&character).unwrap() - 200;
            },
            'L' | 'C' if prev_char == 'X' => {
                integer += roman_numeral_list.get(&character).unwrap() - 20;
            },
            'V' | 'X' if prev_char == 'I' => {
                integer += roman_numeral_list.get(&character).unwrap() - 2;
            },
            _ => {
                integer += roman_numeral_list.get(&character).unwrap();
                prev_char = character;
            }
        };
    }

    return integer;
}



// someone else's code
pub fn other_roman_to_int(s: String) -> i32 {
    s.chars().rfold(0, |acc, c| {
        acc + match c {
            'I' if acc >= 5 => -1,
            'I' => 1,
            'V' => 5,
            'X' if acc >= 50 => -10,
            'X' => 10,
            'L' => 50,
            'C' if acc >= 500 => -100,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    })
}


fn main() {

    let tests = vec![
        "III".to_string(),
        "LVIII".to_string(),
        "MCMXCIV".to_string(),
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let my_time = Instant::now();
        let my_result = my_roman_to_int( test.clone() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_roman_to_int( test.clone() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());

    }

}
