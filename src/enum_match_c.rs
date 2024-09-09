// Matching with Option<T> 
// A function that uses the match expression an Option<i32> 
// 
// Example 

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

 // Here match is use exhausively 


 