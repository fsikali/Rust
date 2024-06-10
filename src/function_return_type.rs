pub fn return_type() { 
 
   let z: i32 = get_sum(5, 10);   

   println!("--------------------------------------rrrrrrrrrrrrrrrrr");
   println!("This is the sum: {}", z); 
   
   let product: i32 = get_product(12); 
   println!("This is the product: {}", product); 

   println!("This is your number: {}", get_number());

   add_numbers(5, 10);    

} 

fn get_sum(x: i32, y: i32) -> i32 { 
    x + y
}
 
fn get_product(x: i32) -> i32 { 
    5 * x
} 

fn get_number() -> i32 { 
     10
} 

fn add_numbers(x: i32, y: i32) { 
    let z: i32 = x + y; 

    println!("This is: {}", x);
    println!("This is: {}", y);
    println!("The sum is: {}", z);
} 


