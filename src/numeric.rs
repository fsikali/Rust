// Data types being used in numeric operations


pub fn numeric_operations() {     
    // addition 
    let value_a: i32 = 10; 
    let value_b: i32 = -20;

    let sum: i32 = value_a + value_b;   

    // subtraction 
    let value_c: u64 = 15; 
    let value_d: u64 = 25; 
    let difference: u64 = value_d - value_c;  

    //multiplication 
    let value_e: f64 = 5.5; 
    let value_f: f64 = 7.5; 
    let product: f64 = value_e * value_f; 

    //Division
    let value_g: i64 = 32; 
    let value_h: i64 = 3;
    let quotient: i64 = value_g / value_h; // integer division truncates towards zero to the nearest integer 

    // Remainder
    let value_i: u128 = 45; 
    let value_j: u128 = 8; 
    let remainder: u128 = value_i % value_j;
    
    println!("------------------------------------");
    println!("Sum equals to: {}", sum); 
    println!("Difference equals to: {}", difference); 
    println!("Product equals to: {}", product); 
    println!("Division equals to: {}", quotient); 
    println!("Remainder equals to: {}", remainder);
}