// Changing a type in shadowing 
// 

pub fn shadowing() { 
    let spaces = "  "; 
    let _spaces = spaces.len(); 

    println!("This is the number of spaces: ");
}