// If we need to concatenate multiple strings, the behaviour of the + operator gets unwiedly 
pub fn my_string() { 
    let s1 = String::from("tic"); 
    let s2 = String::from("tac"); 
    let s3 = String::from("toe"); 

    let s = s1 + "-" + &s2 + "-" + &s3;
} 
 
// s will be tic-tac-toe 
// For complicated string combining, we can instead use the format! macro 
