// To define a Point struct where x and y are both generics but could have different types, 
// we can use multiple generic type parameters 

// Example - A Point <T, U> generic over twon types so that x and y can be values of different types 

struct Point<T, U> { 
    x: T, 
    y: U, 
} 

pub fn my_data_type() { 
    let both_integer = Point{ x: 5, y: 10}; 
    let both_float = Point { x: 1.0, y: 4.0}; 
    let integer_and_float = Point { x: 5, y: 4.0}; 
}  




