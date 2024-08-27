// Iterate over each key/value pair in a hash map using for loop 

use std::collections::HashMap; 

pub fn my_hashmap() { 
    let mut scores = HashMap::new(); 

    scores.insert(String::from("Blue"), 10); 
    scores.insert(String::from("Yellow"), 50); 

    for (key, value) in &scores { 
        println!("{key}: {value}");
    }
} 
 

  