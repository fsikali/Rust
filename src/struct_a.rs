// Define struct and name it Rectangle
struct Rectangle { 
    length: u64,    // Defining the fields as length 
    width: u64,     // and width both of which have u64
} 

pub fn get_area() { 
    // Created a particular instance of the Rectangle that has a 
    // length of 30 and width of 10

    let rect1 = Rectangle { 
        length: 30, 
        width: 10,
    }; 
    
    // The rect_area function accesses the length and width fields of the Rectangle instance
    println!("This is the area: {}", rect_area(&rect1) );
   
} 
 
 // The rect_area function is defined with one parameter -> rectangle whose type
 // is an immutable borrow of a struct Rectangle instance
fn rect_area(rectangle: &Rectangle) -> u64 { 
      rectangle.length * rectangle.width     
}










