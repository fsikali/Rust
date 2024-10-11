// Hash Maps and Ownership 
// For types that implement the Copy trait, like i32, the values are copied into the hashmap 
// For owned values like String, the values will be moved and the hash map will be the owner 
// of those values 

use std::collections::HashMap; 

pub fn my_hashmap() { 
    let field_name = String::from("Favorite color"); 
    let field_value = String::from("Blue"); 

    let mut map = HashMap::new(); 
    map.insert(field_name, field_value);  

    // field_name and field_value are valid at this point, try using them and 
    // see what compiler error you get!
} 



