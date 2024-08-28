// Using a match expression to handle the Result variants that might be returned 

use std::fs::File;

pub fn check_error() { 
    let greeting_file_result = File::open("hello.txt"); 

    let greeting_file = match greeting_file_result { 
        Ok(file) => file, 
        Err(error) => panic!("Problem opening the file : {:?}", error),
    };
} 

// N/B - Like the Option enum, the Result enum and its varinats have been brought into scope 
// by the prelude, so we don't need to specify Result:: before the Ok and Err variants in the match arms 
// 