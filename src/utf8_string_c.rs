// Using the String::from function to create a String from a string literal 

pub fn my_string() { 
    let s = String::from("initial contents");
} 

// Because strings are used for so many things, we can use many different generic 
// APIs for stings, providing us with a lot of options 

// N/B - Strings are UTF-8 encoded, so we include any properly encoded data in them 
 
 