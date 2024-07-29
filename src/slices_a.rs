// String slices is a refrence to a part of a string  
// String literals are string slices already 
// a slice is a kind of reference, so it does not have ownership
// Example 

pub fn my_slice() {
    let a: String =String::from("Hi There"); 

    let b: &str = &a[0..2]; 
    let c: &str = &a[3..9];  

    println!("------------------------------------");
    println!("{b}"); 
    println!("{c}");
}