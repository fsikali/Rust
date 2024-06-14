// Mutable refrences have one big restrictions  
// if you have a mutable reference to a value 
// you can have no other references to that value 

// We can use curly brackets to create a new scope 
// allowing for multiple mutable references, just not simultaneous ones

pub fn mutable_ref() { 
    let mut a: String = String::from("Football Pitch");   
    println!("-------------------------------------"); 

    { 
        let b: &mut String = &mut a; 
        println!("{b}"); 
    } 

    let c = &mut a;
    println!("{c}");

   // let b = &mut a; 
   // let c = &mut a;    // Error cannot borrow 'a' as mutable more than once at a time

   // let b = & a;  no error
   // let c = & a;  no error
   // let d = &mut a; // Error cannot have a mutable refrence while we have an immutable refrence to the same value


}