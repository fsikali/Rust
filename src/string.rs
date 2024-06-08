pub fn my_string() { 
    let firstname: &str = "Flemming";  

    let fname: String = String::from("Flemming");
    
    println!("----------------------------------------------->");
    println!("My first name is: {}", firstname);  
    println!("My name is: {}", fname);
}