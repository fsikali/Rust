// Data type are specified in rust  
// They tell rust which type of data is being specified 
// There are two data type sets scalar and compound  

// Floating point numbers have two types f32 and f64 i.e 32 bits and 64 bits respectively
// All floating point types are signed 

pub fn my_data_type() {
    let value_a: i8 = -12;    
    let value_b: i32 = 25;   // compiler has inferred the type being used based on the value and how we have used it 
    let value_c: u64 = 35;      
    let value_d: i128 = 350;
    let value_e: f32 = 0.02;    // Is a single precision float
    let value_f: f64 = 10.5;    // Is a double precision float

    //let result: f64 = value_e / value_f;

    println!("----------------------------------------------");
    println!("This is 8 bit signed integer type: {}", value_a); 
    println!("This is 32 bit signed integer type: {}", value_b);
    println!("This is 64 bit unsigned integer type: {}", value_c); 
    println!("This is 128 bit signed integer type: {}", value_d);
    println!("This is 32 bit signed  floating point type: {}", value_e); 
    println!("This is 64 bit signed floating point type: {}", value_f);  
    //println!("This is the result: {}", result);
}

