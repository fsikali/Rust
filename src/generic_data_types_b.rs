// In Struct Definitions 
// We can also define structs to use a generic type parameter in one or more fields using the <> syntax 

// Example - A Point<T> struct that holds x and y values of type T 

struct Point<T> { 
    x:T, 
    y:T, 
} 

pub fn my_data_type() { 
    let integer = Point { x: 5, y: 10 }; 
    let float = Point{ x: 1.0, y: 4.0};
} 

// N/B - If we create an instance of a Point<T> taht has values of different types, our code won't compile