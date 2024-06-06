pub fn my_tuple() { 
    let mut elements: (i32, f64, u8) = (250, 8.4, 5);  

    let (x, y, z) = elements;  // Accessing the tuple by destructuring a tuple using pattern matching

    println!("-------------------------------------");
    println!("The value of x is: {}", x); 
    println!("The value of y is: {}", y); 
    println!("The value of z is: {}", z); 

    let x = elements.0;    // Accessing a tuple element directly using the period(.) followed by the index of the value we want to access
    println!("The value of x: {}", x); 

    elements.0 = 125; 
    println!("The value of index zero is: {}", elements.0);
    println!("The value of x is: {}", x);   // Check on this --- why the element didn't change from 250 to 125

} 


