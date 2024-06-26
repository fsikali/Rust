// A message enum whose variants each store different amounts and types of values

enum Message { 
    Quit,
    Move {x: i32, y: i32},
    Write(String), 
    ChangeColor(i32, i32, i32),
}  

pub fn enum_data() { 

}

// This enum has four variants with different types
// Quit has no dta associated with it at all 
// Move has named fields, like a struct does
// Write includes a single String
// ChangeColor includes three i32 values 

// Quiz
// Write the same example using structs














