// Shortcut for Propagating Errors: the ? Operator 
// Example - A function that returns errors to the calling code using the ? operator 

#![allow(unused)] 

use std::fs::File; 
use std::io::{self, Read}; 

pub fn check_error() { 
    fn read_username_from_file() -> Result<String,io::Error> { 
        let mut username_file = File::open("hello.txt")?; 
        let mut username = String::new(); 
        username_file.read_to_string(&mut username)?; 
        Ok(username)
    }
}