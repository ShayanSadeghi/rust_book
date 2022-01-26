// section 10.2

pub trait Summary {
    //traits are like interfaces in C++
    // default implementation
    // call other trait is possible only in the same trait, during a default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    // it is not important that the called trait has a default implementation or not
    fn summarize_author(&self) -> String;
}

pub struct BlogPost {
    pub title: String,
    pub date: String,
    pub author: String,
    pub content: String,
}

impl Summary for BlogPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
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
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
