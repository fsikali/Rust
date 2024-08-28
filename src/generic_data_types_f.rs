// Method Definitions 
// We can implement methods on structs and enums and use generic types in their definitions, too 

// Example - Implementing a method named x on the Point<T> struct that will return a reference to
// the x field of type T 

struct Point<T> { 
    x: T, 
    y: T,
} 

impl<T> Point<T> {
    fn x(&self) -> &T { 
        &self.x
    }
} 

pub fn my_data_type() { 
    let p = Point { x: 5, y:10 }; 

    println!("p.x = {}", p.x());
} 

// 