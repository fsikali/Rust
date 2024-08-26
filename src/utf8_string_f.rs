// Using the + operator to combine two String values into a new String value 

pub fn my_string() { 
    let s1 = String::from("Hello, "); 
    let s2 = String::from("world!"); 
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used 
} 
 
 