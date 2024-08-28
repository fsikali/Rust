// The monomorphized version of the code looks similar to the following  

enum Optioni32 { 
    Some(i32), 
    None,
} 

enum Optionf64 { 
    Some(f64), 
    None, 
} 

pub fn my_data_type() { 
    let integer = Optioni32::Some(5); 
    let float = Optionf64::Some(5.0);
} 

// The generic Option<T> is replaced with the specific definitions created by the compiler. 
// Because Rust compiles generic code into code that specifies the type in each instance, 
// we pay no runtime cost for using generics. 
// When the code runs, it performs just as it would if we had duplicated each definition by hand. 
// The process of monomorphization makes Rust’s generics extremely efficient at runtime. 


