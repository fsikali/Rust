// When the program has a valid reference, the borrow checker enforces the ownership
// and borrowing rules to ensure this reference and any other references to the contents of the
// vector remain valid
// Rule - You can't have mutable and immutable reference in the same scope 

// Example: 
// Attempting to add an element to a vector while holding a reference to an item 
// Compiling this code will result to an error

pub fn my_vector() { 
    let mut v = vec![1, 2, 3, 4, 5];  

    let first = &v[0]; 

    v.push(6); 
    println!("The first element is: {first}");

} 
 
   