#[derive(Debug)]

// Define a struct with fields and data types
struct Student { 
    age: u64,
    //name: String, 
    //school: String, 
    //form: u64,
}

pub fn get_data() {  
    // Created an instance of the student  

    let student_info = Student { 
        age: 12, 
       // name: String::from("Wafula"), 
        //school: String::from("Maseno"),
       // form: 4,
    };
 
     println!("{}", get_details(&student_info)); 
}  


// Create a function to access tha struct 

fn get_details(details: &Student) -> u64 { 
     details.age
} 

// Check refrences, ownership in functions



