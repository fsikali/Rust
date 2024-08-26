// Using a string slice after appending its contents to a string 

pub fn my_string() { 
    let mut s1 = String::from("foo"); 
    let s2 = "bar"; 
    s1.push_str(s2); 
    println!("s2 is {}", s2);
} 

// I the push_str method took ownership of s2, we wouldn't be able to print its value
// on the last line 
