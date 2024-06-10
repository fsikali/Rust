pub fn my_for_loop() { 
    let array: [i32; 10] = [2, 3, 4, 5, 0, 2, 8, 10, 9, 4];  

    println!("------------------------------------------");

    for elements in array { 
        println!("This is my array: {}", elements);
    } 
 
   // function call goes here 
  
  get_array();   
  another_array();
  
}    
 
fn get_array() { 
 
 println!("-------------------------------");

 for number in 1..10 {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}  

fn another_array() { 
   // let price: [i32; 5] = [10, 20, 30, 40, 50]; 
   println!("-------------------------------------------------");

    for elements in 10..50 {  
        println!("This is my name: {}", elements); 

        if elements == 20 { 
            break;
        }
  }
}