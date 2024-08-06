// Reading Elements of Vectors 
// There are two ways to reference a value stored in a vector: via indexing or using the get method
// In the following examples, I've annotated the types of the values that are returned from these functions
// for clarity 

pub fn my_vector() { 
    let v = vec![1, 2, 3, 4, 5]; 

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third { 
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }  
    
} 

// Explanation 
// We use the index value of 2 to get the third element because vectors are indexed by number 
// starting at zero
// Using & and [] gives us a reference to the element at the index value
// When we use the get method with the index passed as an argument, we get an Option<&T> that we can use 
// with match 
// The reason why Rust provides these two ways of reference an element is so you can choose how the program
// behaves when you try to use an index value outside the range of existing elements 
//