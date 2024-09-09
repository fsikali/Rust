// Enumerations also referred to as enums 
// Enums allows definition of a type by enumerating its possible variants 
// Enums give a way of saying a value is one of a possible set of values
// Enum can encode meaning along with data 
// 
// Example of enum -> IP addresses have two variants -> IPv4 and IPv6 

// Defining an enum IpAddrKind
enum IpAddrKind {  
    V4, 
    V6,
}


pub fn my_enum() {  

    // Creating instances of each of the two variants of IpAddrKind 
    // variants of the enums are namespaced under its identifier
    // This is useful because both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // Call the function with either variant 

    route(IpAddrKind::V4); 
    route(IpAddrKind::V6);
} 

// Define a function that takes any IpAddrKind

fn route(ip_kind: IpAddrKind) {} 

// In this example there is no way of storing our IP address data 
// we only know what kind it is. 
// enum_b.rs file shows how to tackle this problem using structs 

struct School { 
    HighSchool
}

enum School { 
    HighSchool, 
    PrimarySchool,
} 

fn main() { 
    let this_school = School::HighSchool;
    let other_school = School::PrimarySchool;
}