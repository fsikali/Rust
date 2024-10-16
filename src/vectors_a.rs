// Storing Lists of Values with Vectors
// Vec<T> - Is a collection type 
// Vectors - they allow storing of more than one value in a single data structure
// that puts all the values next to each other in memory
// They can only store values of the same type  

// Creating a New Vector 
// To create a new empty vector, we call the Vec::new function

pub fn my_vector() { 
    let v: Vec<i32> = Vec::new();
} 

// Explanation: 
// We added a type annotation here because we aren't inserting any value into this vector
// Rust doesn't know what kind of elements we intend to store
// Vectors are implemented using generics
// Vec<T> - is provided by the standard library and can hold any type
// When we create a vector to hold a specific type, we can specify the type within angle brackets
// 
   
    