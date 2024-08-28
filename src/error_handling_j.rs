// Using fs::read_to_string instead of opening and then reading the file 

#![allow(unused)] 

use std::fs; 
use std::io; 

pub fn check_error() { 
    fn read_username_from_file() -> Result<String, io::Error> { 
        fs::read_to_string("hello.txt")
    }
}

