// Change the rules of the game so that nothing else happens on your turn if you roll 
// anything other than a 3 or a 7 
// We can express that by using the unit value(the empty tuple type we mentioned) 

// Example 

pub fn my_match() { 
    let dice_roll;
        match dice_roll { 
            3 => add_fancy_hat(), 
            7 => remove_fancy_hat(), 
            _ => (),
        } 
    fn add_fancy_hat() {} 
    fn remove_fancy_hat() {}
} 

// Explantion:
// Here, we're telling Rust explicitly that we aren't going to use any other value that doesn't
// match a pattern in an earlier arm,and we don't want to run any code in this case
// 



