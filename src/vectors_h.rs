// Iterate over mutable reference to each element in a mutable vector 
// in order to make changes to all elements 

pub fn my_vector() { 
    let mut v = vec![100, 32, 57]; 

    for i in &mut v { 
        *i += 50;
    }
} 

// Explanation: 
// To change the value that the mutable reference refers to, we have to use the * 
// dereference operator to get to the value in i before we can use the += operator
// Iterating over a vector, whetehr immutably or mutably, is safe because of the borrow
// checker's rules 
// If we attempted to insert or remove items in the for loop bodies, we would get a compiler error
// The reference to the vector that the for loop holds prevents simultaneous modification of the whole vector

