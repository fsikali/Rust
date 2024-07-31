// Methods with more Parameters 
// Implementing a second method an the Rectangle struct
//  
// Example
// An instance of Rectangle that takes another instance of Rectangle and
// returns true if the second Rectangle can fit completely within self (the first Rectangle) 
// otherwise, it should return false 
// i.e one we've defined the can_hold method, we want to be able to write the program

#[derive(Debug)] 

struct Rectangle { 
    width: u32, 
    height: u32,
} 

impl Rectangle { 
    fn area(&self) -> u32 { 
        self.width * self.height
    } 

    fn can_hold(&self, other: &Rectangle) -> bool { 
        self.width > other.width && self.height > other.height
    }
}

pub fn my_struct() { 
    let rect1 = Rectangle { 
        width: 30, 
        height: 50,
    }; 

    let rect2 = Rectangle { 
        width: 10,  
        height: 40,
    }; 

    let rect3 = Rectangle { 
        width: 60, 
        height: 45,
    }; 

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); 
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); 

}

// Explanation 
// We define a method, so it will be within the impl Rectangle block 
// The method name is can_hold 
// It takes an immutable borrow of another Rectangle as a parameter 
// We can tell the type of parameter by looking at the code that calls the method 
// rect1.can_hold(&rect2) - passes in &rect2, which is an immutable borrow to rect2, an instance
// of Rectangle 
// 










