// We cannot have a mutable reference while we have an immutable one to the same value. 
// This can be solved by preventing overlapping of the scopes
// Example

pub fn immutabe_ref() { 
    let mut a: String = String::from("Chair");

    println!("------------------------------------"); 

    let b: &String = &a;
    let c: &String = &a; 

    println!("{b}"); 
    println!("{c}");  

    let e: &mut String = &mut a;

    println!("{e}");


}