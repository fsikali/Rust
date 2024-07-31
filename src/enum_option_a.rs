// The Option Enum and Its Advantages Over Null Values
// Option - Is another enum defined by the standard library
// Option type - encode the vary common scenaro in which a value could 
// be something or it could be nothing 
// Option<T> - Is a regular enum 
// Some<T> and None - ara variants of type Option<T> 
// <T> - is a generic type parameter 
// <T> - means that some variant of the Option enum can hold one piece, and that each concrate
// type that gets used in place of T makes the overall Option<T> type a different type
// 
//
//

#![allow(unused)]
enum Option<T> { 
    None, 
    Some(T),
}

pub fn my_enum() { 
    let some_number = Some(5); 
    let some_char = Some('e');

    let absent_number:Option<i32> = None;
}

// Explantion 
// The type of some-number is Option<i32> 
// The some_char is Option<char> which is a different type 
// When we have a Some value, we know that a value is present and the value is held within the Some
// When we have a None value, in some sense it means the same thing as null: we don't have a valid value
// 








