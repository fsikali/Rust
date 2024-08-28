// Cases in Which You Have More Information Than the Compiler 

use std::net::IpAddr; 

pub fn check_error() { 
    let home: IpAddr = "127.0.0.1" 
        .parse() 
        .expect("Hardcorded IP address should be valid");
}
