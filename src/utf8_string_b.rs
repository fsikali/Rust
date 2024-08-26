// Using the to_string method to create a String from a string literal 

pub fn my_string() { 
    let data = "initial contents";

    let s = data.to_string(); 

    // the method also works on a literal directly
    let s = "initial contents".to_string();
} 
 
 