// passing arguments in a function

pub fn my_function() {  

    println!("-----------------------------------------------"); 

    student_age(20, 25, 'A', "Flemming"); 
    //fname("Flemming");
    
} 

fn student_age(age: u64, age_2:u64, grade: char, firstname: &str) { 
    println!("I am {} years old", age); 
    println!("I am {} years old", age_2);   
    println!("This is your grade: {}", grade);

   println!("My name is: {}", firstname);  

   // Check on how to call Strings and use the in Rust 

}