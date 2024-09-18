// The match Control Flow Construct
// match - allows comparing of a value against a series of patterns and then execute code
// based on which pattern matches
// Patterns - can be made up of literal values, variable names, wildcards, and many other things

// Example - Write a function that takes an unknown US coin and, in a similar way as the counting machine,
// determines which coin it is and returns its value in cents

enum Coin { 
    Penny, 
    Nickel,
    Dime, 
    Qurter,
} 

fn value_in_cents(coin: Coin) -> u8 { 
    match coin { 
        Coin::Penny => 1, 
        Coin::Nickel => 5, 
        Coin::Dime => 10, 
        Coin::Quarter => 25,
    }
}
 
pub fn my_match() { 

     let uscoin = Coin::Penny;

     value_in_cents(uscoin);

} 

// Explanation 
// match arm, has two parts - A pattern and some code 
// The first arm - has a pattern that is the value Coin::Penny and then the => operator
// that separates the pattern and the code to run 
// code - In this case is just value 1
// Each arm is separated from the next with a comma 
// When the match expression executes, it compares the resultant value against the pattern
// of each arm,in order. If a pattern matches the values, the code associated with that pattern
// is excuted
// The code associated with each arm is an expression, and the resultant value of the expression
// in the matching arm is the value that gets returned for the entire match expression

// 

         

enum Color { 
    Red, 
    Green, 
    Blue,
} 

fn find_color(my_color: Color) -> String { 
    match my_color { 
        Color::Red => String::from("The color is red"),
        Color::Green => String::from("The color is green"),
        Color::Blue => String::from("The color is blue"),
    }   
} 

pub fn my_enum() { 
    
    let colors = Color::Green; 

    let this_colors = find_color(colors); 

    println!("{}", this_colors);

} 

fn main() { 
    my_enum(); 
    my_match();
}
 
