// We don't typically use curly brackets if the match arm code is short 
// If you want to run multiple lines of code in a match arm, you must use 
// curly brackets, and the comma following the arm is the optional 

enum Coin { 
    Penny, 
    Nickel, 
    Dime, 
    Quarter,
} 

fn value_in_cents(coin: Coin) -> u8 { 
    match coin { 
        Coin::Penny => { 
            println("This is a penny coin"); 
            1
        } 
        Coin::Nickel => 5,
        Coin::Dime => 10, 
        Coin::Quarter => 25,
    }
} 

pub fn uscoins() {  

    let coins = Coin::Penny; 
    let coin_us = value_in_cents(coins); 

    println!("{}", coin_us);
    
} 

fn main() { 
    uscoins();
}
