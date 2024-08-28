// Recoverable Errors with Result 

// Example - Handling Potential Failure with Result 

#![allow(unused)] 
pub fn check_error() { 
    enum Result<T, E> { 
        Ok(T), 
        Err(E),
    }
}  

// The T and E are generic type parameters 
// T - Represents the type of the value that will be returned in a success case within 
// Ok variant 
// E - Represents the type of the error that will be returned in a failure case within the Err 
// variant 
  
// N/B - Because Result has these generic type parameter, we can use the Result type and the functions
// defined on it in many different situations where the successful value and error value we want to
// return may differ 
