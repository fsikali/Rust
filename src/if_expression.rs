// if statement 

pub fn if_statement() { 
    let age: u64 = 35; 
 
 println!("----------------------------------------------");
    if age >= 35 { 
        println!("You are not a youth");
    } else { 
        println!("Your are a youth");
    } 

let score: i32 = get_score(125, 25);  
//println!("{}", sum);  

if score >= 100 { 
    println!("This is the highest score: {}", score); 
} 
else{ 
    println!("The is the lowest score: {}", score);
}

} 

fn get_score(score:i32, score_2:i32) -> i32 {
    score + score_2 
} 


