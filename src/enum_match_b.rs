// Patterns That Bind to Values 
// match arms - they can bind to the parts of the value that match the pattern
// This is how we can extract values out of the enum, variants 

// Example 

#[derive(Debug)]
enum UsState { 
    Alabama, 
    Alaska,
} 

enum Coin { 
    Penny, 
    Nickel, 
    Dime, 
    Quarter(UsState),
} 

fn value_in_cents(coin: Coin) -> u8 { 
    match coin { 
        Coin::Penny => 1, 
        Coin::Nickel => 5,  
        Coin::Dime => 10,
        Coin::Quarter(state) => { 
            println!("Stste quarter from {:?}", state); 
            25
        }
    }
} 
 
 pub fn my_match() { 
    value_in_cents(Coin::Quarter(UsState::Alaska));
 }
