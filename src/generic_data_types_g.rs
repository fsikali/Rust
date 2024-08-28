// An impl block that only applies to a struct with a particular concrete type for the generic
// type parameter T 

struct Point<T> { 
    x: T, 
    y: T,
} 

impl<T> Point<T> {
    fn x(&self) -> &T { 
        &self.x
    }
} 

impl Point<f32> { 
    fn distance_from_origin(&self) -> f32 { 
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
} 

pub fn my_data_type() { 
    let p =Point { x: 5, y: 10}; 
     
    println!("p.x = {}", p.x());
} 

