// Matching with Option<T>
// Instead of comparing coins, we'll compare the variants of Option<T>, but the way
// the match expression works remains the same 
//  
// Example - A function that uses a match expression on an Option<i32> and, if there's a 
// value inside, adds 1 to that value.
// If there isn't a value inside, the function should return the None value and not attempt
// to perform any operations
// 

pub fn my_match() { 
    fn plus_one(x: Option<i32>) -> Option<i32> { 
        match x { 
            None => None, 
            Some(i) => Some(i + 1),
        }
    } 

    let five = Some(5); 
    let six = plus_one(five); 
    let none = plus_one(None);
 } 

 fn main() {} 

 // Explanation: When we call plus_one(five), the variable x in the body of plus_one will
 // have the  Some(5). We then compare that against each match arm;  
 // None => None, 
 // The Some(5) value doesn't match the pattern None, so we continue to the next arm 
 // Some(i) => Some(i + 1), 
 // Consider the second call of plus_one , where x is None. We enter the match and compare to
 // the fisrt arm:
 // None => None
 // It matches! There's no value to add to, so the program stops and returns the None value on the right
 // side of =>. Because the first arm mached, no other arms are compared
 // Combining match and enums is useful in many situations.
  
 // N/B - You'll see this pattern a lot in Rust code: match against an enum, bind a variable to the data
 // inside, and thenexecute code based on it
 //  
 // 

 enum Option<T> { 
    None, 
    Some(e),
 } 

 pub fn my_match() { 
    fn plus_one(x: Option<i32>) -> Option<i32> { 
        match x { 
            Option::None => None, 
            Option::Some => Some(i + e),
        } 

        let five = Some(5);
        let six = plus_one(five); 
        let one  = plus_one(None);
    }
 }



 