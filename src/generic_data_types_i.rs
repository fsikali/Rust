// Perfomance of Code Using Generics 

#![allow(unused)]  

pub fn my_data_type() { 
    let integer = Some(5); 
    let float = Some(5.0);
} 

// When Rust compiles this code, it performs monomorphization. 
// During that process, the compiler reads the values that have been used in Option<T> 
// instances and identifies two kinds of Option<T>: one is i32 and the other is f64.  
// As such, it expands the generic definition of Option<T> into two definitions specialized  
 // to i32 and f64, thereby replacing the generic definition with the specific ones.