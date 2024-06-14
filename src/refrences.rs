pub fn my_refrence() { 
    let mut a: String = String::from("Book");   

    change_value(& mut a);

} 

fn change_value(b: & mut String) { 
    b.push_str("ing");

    println!("{}", b);
}
