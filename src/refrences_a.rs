// Another example of refrencing  

pub fn my_ref() {  

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

}

fn calculate_length(s: &String) -> usize { // s is a refrence to a String 
    s.len()
} 

// When functions have references as parameters instead of the actual values
// we won’t need to return the values in order to give back ownership
// because we never had ownership.

// The action of creating a refrence is called borrowing  



