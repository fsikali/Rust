// Enumerations also referred to as enums 
// Enums allows definition of a type by enumerating its possible variants 
// Enums give a way of saying a value is one of a possible set of values
// Enum can encode meaning along with data 
// 

// Example IP address has two variants -> IPv4 and IPv6
enum IpAddrKind {  // Defining an enum
    V4, 
    V6,
}


pub fn my_enum() {  
    // Creating instances of each of the two variants of IpAddrKind 
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

}