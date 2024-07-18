// The match Control Flow Construct 
// Match allows you compare a value against a series of patterns
// and then execute code based on which pattern matches 

// Example 
// -> A function that takes an unknown Ksh Coin and determines
//  which coin it is and returns its value in cents 

enum Coin { 
    Penny, 
    Nickel, 
    Dime, 
    Quarter,
} 

fn value_in_cents(coin: Coin) -> u8 { 
    match coin {}
}


pub fn my_match() {}