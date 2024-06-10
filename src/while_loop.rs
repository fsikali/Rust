pub fn while_loop() {  

    let mut score: i32 = 100; 
    
    println!("---------------------------------");
    while score != 0 {  
        println!("This is my score: {}", score);
        score += 20;  
        

        if score == 200 { 
            break;
        }
    }  

    println!("Final score is: {}", score); 

// function calling goes here
    
    array_loop();  
    loop_through_array();


}  

// Normal function goes here   
fn array_loop() {  

    println!("---------------------------------------"); 

    let age: [i32; 5] = [10, 20, 30, 40, 50]; 

    let mut my_age: usize = 0; 

    while my_age < 4 {   
        println!("This is my age too: {}", age[my_age]); 
        my_age += 1; 

    }  

    println!("The final score is this: {}", age[my_age]);
}

// Check on calling arrays by passing parameters and returning a value



fn loop_through_array() { 
    let amount: [i32; 5] = [100, 90, 80, 70, 60]; 

    println!("-------------------------------------------");
    let mut index: usize = 0;

    while index < 4 { 
        println!("This is the amount: {}", amount[index]); 
        index += 1;

    } 

    println!("Final amount is: {}", amount[index]);
}









