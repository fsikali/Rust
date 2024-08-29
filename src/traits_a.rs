// Defining a Trait 
// A trait defines functionality a particular type has and shre with other types
// A type’s behavior consists of the methods we can call on that type
// Different types share the same behavior if we can call the same methods on all of those types 
// Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

pub trait Summary {
    fn summarize(&self) -> String;
} 

pub fn my_trait() { 

} 

// Here, we declare a trait using the trait keyword and then the trait’s name, which is Summary
// We’ve also declared the trait as pub so that crates depending on this crate can make use of this trait too 
// Inside the curly brackets, we declare the method signatures that describe the behaviors of the types that implement this trait,
// which in this case is fn summarize(&self) -> String 
// After the method signature, instead of providing an implementation within curly brackets, we use a semicolon 
// Each type implementing this trait must provide its own custom behavior for the body of the method 
// The compiler will enforce that any type that has the Summary trait will have the method 
// summarize defined with this signature exactly 

// A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.