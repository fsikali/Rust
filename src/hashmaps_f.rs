// Adding a Key and Value Only if a Key Isn't Present 
// Example - Using the entry method to only insert if the key does not already have a value 

use std::collections::HashMap; 

pub fn my_hashmap() { 
    let mut scores = HashMap::new(); 
    scores.insert(String::from("Blue"), 10); 

    scores.entry(String::from("Yellow")).or_insert(50); 
    scores.entry(String::from("Blue")).or_insert(50); 

    println!("{:?}", scores);
} 

// The or_insert method on Entry is defined to return a mutable reference to the value for the
// corresponding Entry key if that key exists, and if not, inserts the parameter as the new value 
// for this key and returns a mutable reference to the new value 
// Running the code will print {"Yellow": 50, "Blue": 10} 
// The first call to entry will insert the key for the Yellow team with the value 50 because 
// the Yellow team doesn't have a value already. 
// The second call to entry will not change the hash map because the Blue team already has the 
// value 10 

