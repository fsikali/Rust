// Associated Functions 
// All functions defines within the impl block are called associated functions 
// because they're associated with the named after the impl
// We can define associated functions that don't have self as their first parameter 
// (and thus are not methods) because they don't need an instance of the type to work with
// We've already used one function like this: the String::from funtion that's defined
// on the String type 
// Associated functions that aren't methods are often used for constructors that will return a new
// instance of the struct  
// These are often called new, but new isn't a special name and is't built into the language
// 
// Example 
// We could choose to provide an associated function named square that would have one 
// dimension parameter and use that both with height, thus making easier to create 
// a square  Rectangle rather than having to specify the same value twice

#[derive(Debug)] 

struct Rectangle { 
    width: u32, 
    height: u32,
} 

impl Rectangle { 
    fn square(size: u32) -> Self { 
        Self { 
            width: size, 
            height: size,
        }
    }
} 

pub fn my_struct() { 
    let sq = Rectangle::square(3);
}


// Explanation 
// The Self keywords in the return type and in the body of the function are aliases for the
// type that appears after the impl keyword, which in this case is Rectangle 
//  To call this associated function, we use :: syntax with the struct name; let sq = Rectangle::square(3); 
// is an example. This function is namespaced by the struct: the :: syntax is used for both associted functions 
// and namespaces created by modules


