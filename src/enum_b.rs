// How to add data in enum using structs  

// Define a struct that has two fields and a type 
// Two instances
// Representing the same concept using just enum is more concise

enum IpAddrKind { 
    V4,
    V6,
} 
 
 
struct IpAddr { 
    kind: IpAddrKind, 
    address: String,
} 

pub fn enum_data() { 
    let home = IpAddr { 
        kind: IpAddrKind::V4, 
        address: String::from("127.0.0.1"),
    }; 

    let loopback = IpAddr { 
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
} 






