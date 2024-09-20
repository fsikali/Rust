// Catch-all Patterns and the_Placeholder 
// Using enums, we can also take special actions for few particular values, but for all
// other values take one default action 

// Example - catch-all pattern
// In a game -Implement a game where, if ou roll a 3 in a dice roll, your player doesn't 
// move, but instead gets a new fancy hat, if you roll a 7, your player loses a fancy hat
// For all other value, your player moves that number of spaces on the game board 

// Example - Here is a match that implements that logic, with the result of the dice roll hardcorded
// rather than a random value, and all other logic represented by functions without bodies because 
// actually implementing them is out of scope for this example:

pub fn my_match() { 
    let dice_roll = 9; 
    match dice_roll { 
        3 => add_fancy_hat(), 
        7 => remove_fancy_hat(), 
        other => move_player(other),
    } 

    fn add_fancy_hat() {} 
    fn remove_fancy_hat() {} 
    fn move_player(num_spaces: u8) {}
} 


// Explanation 
// First two arms -the patterns are the literal values 3 and 7 
// last arm - covers every possible value, the pattern is the variable we have choosen to name other 
// The code that runs for the othr arm uses the variable by passing it to the move_player function 
// This code compiles, even though we haven't listed all the possible values a u8 can have because 
// the last pattern will  match all values not specifically listed.
// This catch-all pattern meets the requirement that match must be exhaustive
// Note that we have to put the catch-all arm last because the patterns are evaluated in order 
// If we put the catch-all arm earlier, the other arms would never run, so Rust will warn us if we add
// arms after caatch-all


