// Abstracted code to find the largest number in two lists 

pub fn generic_type() { 
    let number_list = vec![34, 50, 25, 100, 65]; 

    let result = largest(&number_list); 
    println!("The largest number is {}", result); 
    assert_eq!(*result, 100);  

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8]; 

    let result = largest(&number_list); 
    println!("The largest number is {}", result); 
    assert_eq!(*result, 6000);
} 

fn largest(list: &[i32]) -> &i32 { 
    let mut largest = &list[0]; 

    for item in list { 
        if item >  largest { 
            largest = item;
        }
    } 

    largest
} 

// The largest function has a parameter called list, which represents any concrete slice of i32  
// values we might pass into the function. 
// Steps used to change the code from generic_b.rs to generic_c.rs:
// 1. Identify duplicate code. 
// 2. Extract the duplicate code into the body of the function and specify the inputs  
      // and return values of that code in the function signature.
// 3. Update the two instances of duplicated code to call the function instead.