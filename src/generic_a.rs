// Removing duplication by Extracting a Function 
// Example - Finding the largest number in a list of numbers 

pub fn generic_type() { 
    let number_list = vec![34, 50, 25, 100, 65]; 

    let mut largest = &number_list[0]; 

    for number in &number_list { 
        if number > largest { 
            largest = number;
        }
    } 

    println!("The largest number is {}", largest); 
    assert_eq!(*largest, 100);
} 


