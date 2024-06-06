pub fn my_tuple() { 
    let age: (u64, u64, u64) = (25, 35, 45);  
    
    let my_age: u64 = age.0;  
    let student_age: u64 = age.2; 

    println!("---------------------------------"); 
    println!("This is the student age: {}", student_age);
    println!("This is my age: {}", my_age); 
    
    println!("--------------------------------");

    let (x, y, z) = age; 

    println!("This is your age: {}", x);
    println!("This is your age: {}", y);
    println!("This is your age: {}", z);
}