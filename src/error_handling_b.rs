// Calling a function that returns a Result value because the function could fail
// Example - Opening a file 

use std::fs::File; 

pub fn check_error() { 
    let greeting_file_result = File::open("hello.txt");
} 

// The return type of File::open is a Result<T, E> 
// The generic parameter T has been filled in by the implementation of File::open with
// the type of the success value, std::fs::File, which is file handle. 
// The type of E used in the error value is std::io::Error 
// File::open function needs to have a way to tell us whether it succeeded or failed and at the same
// time give us either the file handle or error information. This information is exaclty what the Result 
// enum conveys. 
// NB - In the case where the File::open succeeds, the value in the variable greeting_file_result will be an 
// instance Ok that contains s file handle. In the case where it fails, the value in greeting_file_result will
// be an instance of Err that contains more information about the kind of error that happened. 
 

















