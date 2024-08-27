// Common collections - Hash Maps 
// Hash maps are useful when you want to look up data not by using an index, as you 
// can with vectors, but by using a key that can be any type. 

// Example - Creating a New Hash Map and inserting some keys and values

use std::collections::HashMap;

pub fn my_hashmap() { 
    let mut scores = HashMap::new();  

    scores.insert(String::from("Blue"), 10); 
    scores.insert(String::from("Yellow"), 50);
} 

// Just like vectors, hash maps store their data on the heap 
// This HashMap has keys of type String and values of type i32
// LIke vectors, hash maps are homogeneous: all the keys must have the same
// type as each other, and all of the values  must have the same types

