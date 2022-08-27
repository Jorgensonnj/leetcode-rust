use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut queue = "".to_string();

    //println!("{}", queue);

    let mut hash: HashMap<String, i32> = HashMap::new();

    let mut length = 0;
    for (i, char) in s.chars().enumerate() {
        if !hash.contains_key(&char.to_string()) {
            hash.insert(char.to_string(), i as i32 );
            queue.push(char);
        } else {
            length = if queue.len() > length { queue.len() } else { length };
            hash.clear();
            hash.insert(char.to_string(), i as i32 );
        }

    };


    todo!()
}

fn main() {
    println!("Hello, world!");
}
