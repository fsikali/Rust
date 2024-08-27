// Updating a Value Based on the Old Value 

// Example - Counting occurrences of word using a hash map that stores words and counts 

use std::collections::HashMap; 

pub fn my_hashmap() { 
    let text = "hello world wonderful world"; 

    let mut map = HashMap::new(); 

    for word in text.split_whitespace() { 
        let count = map.entry(word).or_insert(0); 
        *count += 1; 

    } 

    println!("{:?}", map); 
} 

// This code will print {"world": 2, "hello": 1, "wonderful": 1}
// The split_whitespace method returns an iterator over sub-sclices, separated by whitespace, 
// of the value in text. 
// The or_insert method returns a mutable reference (&mut V) to the value for the specified key 
// Here we store the mutable reference in the count variable, so in order to assign to that value, 
// we must frst deference count using the asterisk (*). 
// The mutable reference goes out of scope at the end of the for loop, so all of these changes are 
// safe and allowed by the borrowing rules. 