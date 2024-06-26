// The Option enum 
// Option type encodes the very common scenario in which a value
// could be something or it could be nothing 


enum Option<T> { 
    None, 
    Some(T),
}
 
// <T> means that 'Some' variant of the 'Option' enum can hold one 
// piece of data of any type and that each concrete type that gets
// used in place of T makes the overall Option<T> type a different 

// Example using 'Option' values to hold number types and string types

let some_number = Some(5); 
let some_char = Some('e'); 

let absent_number: Option<i32> = None;


// The type of 'some_number' is Option<i32> 
// The type of 'some_char' is Option<char> which is a different type
// Rust can infer these types because we've specified a value inside the 
// Some variant 
// For 'absent_number' Rust requires us to annote the overall Option type: 
// the compiler can't infer the type that the corresponding Some variant
// will hold by looking only at a 'None' value 
// Here, we tell Rust that we mean for 'absent_number' to be of type Option<i32>

/* 
   When we have a 'Some' value, we know that a value is present and the value
   is held within the 'Some'
   When we have a 'None' value, in some sense it means the same thing as null: 
   we don't have a valid value.
   So why is having Option<T> any better than having null? 

   In short, becaue Option<T> and T (where T can be any type) are different
   types, the compiler won't let use an Option<T> value as if it were definitely 
   a valid value. 
*/
 
 //  For example, this code won't compile, because it's tying to add
 //  an i8 to an Option<i8>  

 let x: i8 =5; 
 let y: Option<i8> = Some(5); 

 let sum = x + y;

// This error message means the Rust doesn't understand how to add an i8 
// and an Option<i8>,  because they're different types
/* 
  When we have a value of type like i8 in Rust, the complier will ensure that 
  we always have a valid value
  We can proceed confidently without having to check for null before using that
  value. 
  Only when we have an Option<i8> (or whatever type of value we're working with) 
  do we have to worry about possiblly not having a value, and the compiler will make
  sure we handle that case before using the value. 
  In other words, you have to convert an Option<T> to a T before you can perform 
  T operations with it. Generally, this helps catch one of the most common issues
  with null: assuming that something isn't null when it actually is.

  Eliminating the risk of incorrectly assuming a not-null value helps you to be 
  more confident in your code. In order to have a value that can possibly be null,
  you must explicitly opt in by making the type of that value Option<T>. 

*/




















