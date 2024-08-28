// Matching on Different Errors 
// Example - Handling different kinds of errors in different ways 

use std::fs::File; 
use std::io::ErrorKind; 

pub fn check_error() { 
    let greeting_file_result = File::open("hello.txt");  

    let greeting_file = match greeting_file_result { 
        Ok(file) => file, 
        Err(error) => match error.kind() { 
            ErrorKind::NotFound => match File::create("hello.txt") { 
                Ok(fc) => fc, 
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }, 
            other_error => { 
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
} 

// The type of the value that File::open returns inside thr Err variant is io::Error, which is a struct 
// provided by the standard library. 
// This struct has a method kind that we can call to get an io::ErrorKind value 
// The enum io::ErrorKind is provided by the standard library and has variants representing the different 
// kinds of errors that might result from an io operation 
// The variant we want to use is ErrorKind::NotFound, which indicates the file we're trying to open doesn't exist
// yet, so we match on greeting_file_result, but we also have an inner match on error.kind()
//