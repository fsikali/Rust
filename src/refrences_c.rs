// Just as variables are immutable by default so are references 
// We are not allowed to modify something we have a refrence to 
// But we can use mutable refrence to allow modification of a borrowed value 



pub fn my_ref() {  
    
    println!("---------------------------------------------");

    let mut company: String = String::from("Toyota"); 

    modify(&mut company);  

   // let a = &mut company; 
   // let b = &mut company;    // Check on this mutable refrence 

   // let a = &company;  no error
   // let b = &company;  no error
   // let c = &mut company; // Error

   // println!("{company}");

}

fn modify(add_details: &mut String) {
    add_details.push_str(" Manufacturing Company");      // Modifying 

    println!("{add_details}");
}