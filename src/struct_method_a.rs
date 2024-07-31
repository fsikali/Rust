// We can choose to give a method the same name as one of the struct's fields
// We can define a method on Rectangle that is also named width

#[derive(Debug)]

struct Rectangle { 
    width: u32, 
    height: u32,
} 

impl Rectangle { 
    fn width(&self) -> bool { 
        self.width > 0
    }
} 

pub fn my_struct() { 
    let rect1 = Rectangle { 
        width: 30,
        height: 50,
    }; 

    if rect1.width() { 
        println!("The rectangle has a nonzero width; it is {} ", rect1.width);
    }
}


// Here, we're choosing to make the width method return true if the value in thr instance's widht field
// is greater than 0 and false if the value is 0










































