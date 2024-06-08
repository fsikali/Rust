pub fn my_loop() { 
    println!("----------------------------------------------");  

    let mut weight: i64 = 75;

    let result: i64 = loop {  
        weight += 1; 
        println!("The weight is: {}", weight);
         if weight == 80 { 
            break weight * 2;
         } 
    }; 

   
     println!("This is the result: {}", result);  

     // function call goes here 

     another_loop(); 
     test_loop(); 
     //loop_test();
} 

fn another_loop() { 
    let mut score: i32 = 0; 

    println!("-------------------------------------------------");

    let my_score: i32 = loop {  
        score += 10;
        println!("This is your score: {}", score); 
        
        
    
       if score == 50 { 
        break score * 2;
       } 
    }; 
    
     println!("This is your final score: {}", my_score);

} 

fn test_loop() { 
    let mut input = 2; 

    println!("-------------------------------------------------");  

    let final_input: i32 = loop {  
        input += 2;
        println!("Your input is: {}", input);
          

        if input == 20 { 
            break input;  // check on the java implementation of the break keyword
        }
    }; 

    println!("Your final input is: {}", final_input);

} 

/* 


fn loop_test() { 
    let mut counter: i32 = 0; 
     
    println!("------------------------------------"); 

    loop { 
        counter += 5; 

        println!("{counter}");
    }
}


*/


// More revision 
// Why we are createing the variable >>> result <<< to hold the value returned from the loop 
// Loops in rust don't implement the length of the iteration in their definition directly instead they use if and break keyword
// to specify the actual length of the iteration