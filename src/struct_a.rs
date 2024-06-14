// Define struct and name it Rectangle
struct Rectangle { 
    length: u64,    // Defining the fields as length 
    width: u64,     // and width both of which have u64
} 

pub fn get_area() { 
    // Created a particular instance of the rectangle whose type is an immutable 
    // borrow of a struct Rectangle instance 

    let rect1 = Rectangle { 
        length: 30, 
        width: 10,
    }; 

    println!("This is the area: {}", rect_area(&rect1) );
   
} 
 
fn rect_area(rectangle: &Rectangle) -> u64 {  // The rect_area function accesses the length and the width
      rectangle.length * rectangle.width      // fields of the Rectangle instance
}










