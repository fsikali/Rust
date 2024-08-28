// Propagating Errors
// When a function's implementation calls something that might fail, instead of handling the error
// within the function itself, you can return the error to the calling code so that it ccan decide what 
// to do . This is known as propagating the error and gives more control to the calling code, where there
// might be more information or logic that dictates how the error should be handled than what you have 
// available in the context of your code

// Example - A function that returns errors to the calling code using match 

#![allow(unused)]
use std::fs::File; 
use std::io::{self, Read}; 

pub fn check_error() { 
    fn read_username_from_file() -> Result<String, io::Error> { 
        let username_file_result = File::open("hello.txt"); 

        let mut username_file = match username_file_result { 
            Ok(file) => file, 
            Err(e) => return Err(e),
        }; 

        let mut username = String::new(); 

        match username_file.read_to_string(&mut username) { 
            Ok(_) => Ok(username), 
            Err(e) => Err(e),
        }
    }
} 

// Result<String, io::Error> - This means the function is returning a value of the type Result<T, E> 
// where the generic parameter T has been filled in with the concrete type String, and the generic type E
// has been filled in with the concrete type io::Error 
