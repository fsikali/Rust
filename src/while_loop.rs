pub fn while_loop() {  

    let mut score: i32 = 200; 
    
    println!("---------------------------------");
    while score != 0 {  
        score -= 20;  
        println!("This is my score: {}", score); 
        

        if score == 80 { 
            break;
        }
    }  

    println!("Final score is: {}", score); 

// function calling goes here
    

}  

// Normal function goes here  












