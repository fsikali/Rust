// You can declare a new variable with the same name as a previous variable 
// In rust this is called shadowing

pub fn shadowed_var() {
    let  y: i32 = 10; 

    let y: i32 = y + 10; 

    { 
        let y: i32 = y * 5; 
        println!("--------------------------------------------"); 
        println!("The value of y in the inner scope is: {} ", y); 
    }

    println!("The value of y is: {}", y);
}