// In rust variables are immutable by default (They can't change) 
// You can make variables mutable by adding mut in front of the variable name

pub fn immutable_variable() { 
    let mut x : i32 = 5; 
    println!("The value of x is: {x}");  

    x  = 10; 
    println!("The value of x is: {x}"); 

    immutable_var();
} 

fn immutable_var() { 
    let mut y : i32 = 20; 
    println!("The value of y is: {y}"); 

    y = 25; 
    println!("The value of y is: {y}");
}



