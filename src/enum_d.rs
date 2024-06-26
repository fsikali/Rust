// How the standard library defines IpAddr
// It has the exact enum and variants in the form of two different structs
// which are defined for each variant

struct Ipv4Addr { 
    // --snip--
}

struct Ipv6Addr { 
    // --snip--
}

enum IpAddr { 
    V4(Ipv4Addr), 
    V6(Ipv6Addr),
}

pub fn enum_data() { 

} 

// This code illustrates that you can put any kind of data inside an
// enum variants: strings, numeric types or structs 
// You can even include another enum 
// Standard  library types are often not much more complicated than you 
// might come up with  
// -> NB: Even though the standard library contains a definition for IpAddr
// we can still create and use our own definition conflict because
// we haven't brought the standard library's definition into our scope


























