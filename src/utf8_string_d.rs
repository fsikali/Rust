// Appending to a String with push_str and push

pub fn my_string() { 
    let mut s = String::from("foo"); 
    s.push_str("bar");
} 

// The push_str method takes a string slice because we don't necessarily
// want to take ownership pf the parameter