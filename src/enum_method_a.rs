#[derive(Debug)]
enum Message { 
    Quit, 
    Move {x: i32, y: i32},
    Write(String), 
    ChangeColor(i32, i32, i32),
} 

impl Message { 
    fn call(&self) { 
        // method body would be defined here
    }
} 

pub fn my_enum() { 
     let m = Message::Write(String::from("Hello"));
     m.call();
} 

// Explanation 
// The body of the method would use self to get the value that we called
// the method on.
// Created a variable m that has the value Message::Write(String::from("Hello")) 
// and that is what self will be in the body of the call method when m.call() runs

