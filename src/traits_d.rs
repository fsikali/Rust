// Default Implementations 
// Example - Defining a Summary trait with a default implementation of the summarize method 

pub trait Summary { 
    fn summarize(&self) -> String { 
        String::from("(Read more...)")
    }
} 

pub struct NewsArticle { 
    pub headline: String, 
    pub location: String, 
    pub author: String, 
    pub content: String,
} 

impl Summary for NewsArticle {} 

pub struct Tweet { 
    pub username: String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool,
} 

impl Summary for Tweet { 
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


// To use a default implementation to summarize instances of NewsArticle, 
// we specify an empty impl block with impl Summary for NewsArticle {}

// Even though we're no longer defining the summarize method on NewsArticle directly, we've 
// provided a default implemetation and specified that NewsArticle implements the Summary trait 
// As a result, we can still call the summarize method on an instance of NewsArticle, like this:

use aggregator::{self, NewsArticle, Summary};

pub fn my_trait() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
} 

// This code prints New article available! (Read more...) 

// Creating a default implementation doesn’t require us to change anything about the implementation 
// of Summary on Tweet in trait_b.rs 
// The reason is that the syntax for overriding a default implementation is the same as the syntax 
// for implementing a trait method that doesn’t have a default implementation.



