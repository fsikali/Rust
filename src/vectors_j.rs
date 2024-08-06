// Dropping a Vector Drops Its Elements 
// Like any other struct, a vector id freed when it goes out of scope, as annotated

pub fn my_vector() { 
    { 
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    }   // <- v goes out of scope and is freed here
} 

// Explanation: 
// When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds
// will be cleaned up
// The borrow checker ensures that any references to contents of a vector are only used while the vector
// itself is valid







