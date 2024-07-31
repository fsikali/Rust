// Rust has a pattern that we can use when we want a catch-all but don't
// want to use the value in the catch-all-pattern 
// Example: _ 

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






