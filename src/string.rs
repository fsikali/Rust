pub fn my_string() { 
    let firstname: &str = "Flemming";  

    let fname: String = String::from("Flemming");
    
    println!("----------------------------------------------->");
    println!("My first name is: {}", firstname);  
    println!("My name is: {}", fname); 

    add_string(); 
    clone_string();
  
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



