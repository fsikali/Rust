// How a binary crate could use our aggregator library crate 

use aggregator::{Summary, Tweet}; 

pub fn my_trait() { 
    let tweet = Tweet { 
        username: String::from("horse_ebooks"), 
        content: String::from( 
            "of course, as you probably already know, people",
        ), 
        reply: false, 
        retweet: false,
    }; 
     
    println!("1 new tweet: {}", tweet.summarize());
} 

