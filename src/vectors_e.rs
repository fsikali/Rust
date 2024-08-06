// Example 
// Test what happens when we have a vector of five elements and then try to access
// an element at index 100 with each technique

pub fn my_vector() { 
    let v = vec![1, 2, 3, 4, 5]; 

    let does_not_exist = &v[100]; 
    let does_not_exist = v.get(100);
} 

// Explanation: 
// When we run this code, the first [] method will cause the program to panic because
// it references a nonexistent element.
// This method is best used when you want your program to crash if there's an attemt to access an element
// past the end of the vector 

// When the get method is passed an index that is outside the vector, it returns None without
// panicking
// You should use this method if accessing an element beyond the range of the vector may happen 
// occasionally under normal circumstances
// Your code will then have logic to handle having either Some(&element) or None

