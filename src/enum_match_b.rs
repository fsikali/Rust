// Patterns That Bind to Values 
// match arms - they can bind to the parts of the value that match the pattern
// This is how we can extract values out of the enum, variants 

// Example - Lets change one of our enum variants to hold data inside it
//         - From 1999 through 2008, the United States minted quarters with different designs
//           for each of the 50 states on one side. No other coins got state designs, so only  quarters 
//           have this extra value. We can add this information to our enum by changing the Quarter variant 
//           to include a UsState values stored inside it, which we've done

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
            println!("State quarter from {:?}", state); 
            25
        }
    }
} 
 
 pub fn my_match() {  

    let this_coins = value_in_cents(Coin::Quarter(UsState::Alaska));   
    //let coins_value = value_in_cents(this_coins); 

    println!("Coin value: {}", this_coins); 

 }

 