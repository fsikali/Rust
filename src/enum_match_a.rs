// The match Control Flow Construct
// match - allows comparing of a value against a series of patterns and then execute code
// based on which pattern matches
// Patterns - can be made up of literal values, variable names, wildcards, and many other things

// Example   

// Write a function that takes an unknown US coin and, in a similar way as the counting
// machine, determines which coin it is and returns its value in cents

enum Coin { 
    Penny, 
    Nickel,
    Dime, 
    Qurter,
} 

fn value_in_cents(coin: Coin) -> u8 { 
    match coin {
        Coin::Penny => { 
            println!("Congrates"); 
            1
        }
        Coin::Nickel => 5, 
        Coin::Dime => 10, 
        Coin::Qurter => 25,
    }
} 

pub fn my_match() {} 

// Explanation 
// match arm, has two parts - A pattern and some code 
// The first arm - has a pattern that is the value Coin::Penny and then the => operator
// that separates the pattern and the code to run 
// code - In this case is just value 11
// 
