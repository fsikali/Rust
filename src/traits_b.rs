// Implementing a Trait on a Type 
// Example - Implementing the Summary trait on the NewsArticle and Tweet types 

pub trait Summary { 
    fn summarize(&self) -> String;
} 

pub struct NewsArticle { 
    pub headline: String, 
    pub location: String, 
    pub author: String, 
    pub content: String, 
} 

impl Summary for NewsArticle { 
    fn summarize(&self) -> String { 
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
} 

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

pub fn my_trait() { 
    
}

// Implementing a trait on a type is similar to implementing regular methods 
// The difference is that after impl, we put the trait name we want to implement, 
// then use the for keyword, and then specify the name of the type we want to implement the trait for. 
// Within the impl block, we put the method signatures that the trait definition has defined.  
// Instead of adding a semicolon after each signature, 
// we use curly brackets and fill in the method body with the specific behavior 
// that we want the methods of the trait to have for the particular type. 


