// Ownweship in rust
// using push_str method and String type

pub fn my_string() {  

    let mut firstname: String = String::from("Flemming");  

    firstname.push_str("Sikali");
    
    println!("----------------------------------------------->");
    println!("My first name is: {}", firstname);  
 

    add_string(); 
    clone_string(); 
    string_type(); 
    string_clone();
  
} 

fn  add_string() {  
    let mut school_name: String = String::from("Technical "); 

    school_name.push_str(" University");  
    school_name.push_str(" of"); 
    school_name.push_str(" Mombasa");

    println!("My campus is: {}", school_name);
   
}  

// Variables and data interacting with clone 

fn clone_string() { 
    let  car_name: String = String::from("Mercedez"); 

    let another_car_name: String = car_name.clone(); 

    println!("------------------------------------------------");
    println!("This is the car name: {}", car_name); 
    println!("This is another car name: {}", another_car_name);
}    

// Moving ownership 

fn string_type() { 
    let  first_phone: String = String::from("Samsung"); 
    let another_phone: String = first_phone; 

    println!("Another phone is: {}", another_phone); 

}

// Clone allows copying directly from the heap

fn string_clone() { 
    let first_phone: String = String::from("OPPPO"); 
    let another_phone: String = first_phone.clone();

    println!("My other phone: {}", another_phone);
    println!("First phone: {}", first_phone);
}











































