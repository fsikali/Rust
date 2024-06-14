// Example of refrencing

pub fn my_ref() { 

    let s1 = String::from("hello");

    calculate_length(&s1);

    
}

fn calculate_length(s: &String) -> usize {
   let length = s.len();  

   println!("The length of '{}' is {}.", s, length);

   length
}
