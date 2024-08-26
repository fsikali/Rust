// Storing UTF-8 Encoded Text with Strings 
// Rust has only one string type in the core language, which is the string slice str 
// that is usually seen in its borrowed form &str 
// The String type, which is provided by Rust's standard library rather than coded 
// into the core language, is growable, mutable, owned, UTF-8 encoded string type 
// N/B - String literals, for example, are stored in the program's binary and are
// therefore string slices 


// Example - Creating a new empty String 

pub fn my_string() { 
    let mut s = String::new();
} 






