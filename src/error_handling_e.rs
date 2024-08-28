// Shortcuts for Panic on Error: unwrap and expect 

use std::fs::File; 

pub fn check_error() { 
    let greeting_file = File::open("hello.txt").unwrap();
} 

// The unwrap method is a shortcut method implemented just like the match expression 
// If we run this code without a hello.txt file, we'll see an error message from the panic! call 
// that the unwrap method makes 
