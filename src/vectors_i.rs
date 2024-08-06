// Using an Enum to Store Multiple Types
// Vectors can only store values that are of the same type 
// When we need one type to represent elements of different types, we can define an enum 
// 
// Example: 
// Say we want to get values from a row in a spreadsheet in which some of the columns in the 
// row contain integers, some floating-point numbers, and some strings. We can define an enum
// whose variants will hold the different value types, and all the enum variants will be
// considered the same type: that of the enum. Then we can create a vector to hold that enum 
// and so, ultimately, holds different types

enum SpreadsheetCell { 
    Int(i32), 
    Float(f64), 
    Text(String),
} 

pub fn my_vector() { 
    let row = vec![SpreadsheetCell::Int(3), 
    SpreadsheetCell::Text(String::from("blue")), SpreadsheetCell::Float(10.12)];
}




