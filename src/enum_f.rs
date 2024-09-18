// Just as we have been able to define methods on structs using impl 
// we're also able to define methods on enums. 

// Example -> Here's a method named call that we could define on our message enum

enum Message { 
    Quit, 
    Move { x: i32, y: i32 }, 
    Write(String), 
    ChangeColor(i32, i32, i32),
} 

impl Message {
    fn call(&self)  {  // check how to return in such a scenario
        // self.Write
        // method body would be defined here
    }
}

pub fn my_enum() {  

    let m = Message::Write(String::from("hello"));
    m.call();
} 

// The body of the method would use self to get the value that we called 
// the method on 






