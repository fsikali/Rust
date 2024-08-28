// Changing main to return Result<(), E> allows the use of the ? operator on Result values

use std::error::Error; 
use std::fs::File; 

pub fn check_error() -> Result<(), Box<dyn Error>> { 
     let greeting_file = File::open("hello.txt")?; 

     Ok(())
} 

