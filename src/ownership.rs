pub fn my_ownership() { 

        let s1: String = String::from("hello");   
    
        let (s2, len) = calculate_length(s1);
    
        println!("The length of '{}' is {} .", s2, len);
    }

fn calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len(); // len() returns the length of a String
    (s, length)
}  



















