// Overwriting a Value 
// Replacing a value stored with a particular key 

use std::collections::HashMap; 

pub fn my_hashmap() { 
    let mut scores = HashMap::new(); 

    scores.insert(String::from("Blue"), 10); 
    scores.insert(String::from("Blue"), 25); 

    println!("{:?}", scores);
} 

// This code will print {"Blue": 25}. The original value of 10 has been written 
  