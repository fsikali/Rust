// Shortcuts for Panic on Error: unwrap and expect 
// The expect method lets us also choose the panic! error message 
// Using expect instead on unwrap and providing good error messages can convey 
// your intent and make tracking down the source of a panic easier 

use std::fs::File; 

pub fn check_error() { 
    let greeting_file = File::open("hello.txt") 
    .expect("hello.txt should be included in this project");
} 

// We use expect in the same way as unwrap: to return the file handle or call the panic! macro. 
// The error message used by expect in its call to panic! will be the parameter that we pass to
// expect, rather than the default panic! message that unwrap uses.