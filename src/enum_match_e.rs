// Rust has a pattern that we can use when we want a catch-all but don't want to use the value in the
// catch-all pattern: 
// _ is a special pattern that matches any value and does not bind to that value. 
// This tells Rust we aren't going to use the value, so Rust won't warn us about an unused variable 

// Example - If you roll anything other than a 3 or 7, you must roll again. We no longer need to use 
// the catch-all value, so we can change our code to use _ instead of the variable named other

pub fn my_match() { 

let dice_roll = 9; 
match dice_roll { 
    3 => add_fancy_hat(), 
    7 => remove_fancy_hat(), 
    _=> reroll(),
} 

fn add_fancy_hat() {} 
fn remove_fancy_hat() {} 
fn reroll() {}

}

// This example also meets the exhaustiveness requirements because we're explicitly ignoring
// all other values in the last arm; we haven't forgotten anything






