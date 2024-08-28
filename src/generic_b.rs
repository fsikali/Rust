// Code to find the largest number in two lists of numbers 

pub fn generic_type() { 
    let number_list = vec![34, 50, 25, 100, 65]; 

    let mut largest = &number_list[0]; 

    for number in &number_list { 
        if number > largest { 
            largest = number;
        }
    } 

    println!("The largest number is {}", largest);  

    let number_list = vec![102, 34, 6000, 89, 54, 2 , 43, 8]; 

    let mut largest = &number_list[0]; 

    for number in &number_list { 
        if number > largest { 
            largest = number;
        }
    } 

    println!("The largest number is {}", largest);
}


// To eliminate this duplication, we’ll create an abstraction by defining a function that 
// operates on any list of integers passed in a parameter 
// This solution makes our code clearer and lets us express the concept of finding the  
// largest number in a list abstractly.