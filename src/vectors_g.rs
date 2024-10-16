// Iterating over the Values in a Vector
// To access each element in a vector in turn, we would iterate through all 
// of the elements rather than use indices to access one at a time 
// 
// Example: 
// How to use a for loop to get immutable references to each element in a vectors of i32 
// values and print them 

pub fn my_vector() { 
    let v = vec![100, 32, 57]; 
    for i in &v { 
        println!("{i}");
    }
}


