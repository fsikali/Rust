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

pub fn get_area() { 
    let rect1 = Rectangle { 
        width: 30, 
        height: 50,
    };

    println!("The area is:{}", rect1.area());
}











































