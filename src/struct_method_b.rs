// Method Syntax 
// Methods are similar to functions: we declare them with the fn keyword and a name 
// They can have parameters and a return value 
// They contain some code that run when the method is called from somewhere else 
// Unlike functions, methods are defined within the context of a struct (or an enum or a trait
// object) 
// Their first parameter is always self, which represents the instance of the struct the method 
// is being called on
//
//
// Example Defining Methods

#[derive(Debug)] 


 struct Rectangle { 
    width: u32, 
    height: u32,
 } 

impl Rectangle { 
    fn area(&self) -> u32 { 
        self.width * self.height
    }
} 

pub fn my_struct() { 
    let rect1 = Rectangle { 
        width:  30,
        height: 50,
    }; 

    println!("The area of the rectangle is {} square pixels", rect1.area());
}


// To define the function within the context of Rectanlge, we start an impl (implementation) block
// for Rectangle
// Everyting within this impl block will be associated with the Rectangle type
// &self is usually short for self: &Self 
// Main reason for using methods instead of functions, in addition to providing method syntax and
// not having to repeat the type of self in every method's signature, is for organization
//
// 





























