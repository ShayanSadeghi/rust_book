// section 10.2
use myTraits::{BlogPost, Summary, Tweet}; //we should import Summary too
fn main() {
    let my_tweet = Tweet {
        username: String::from("some_un"),
        content: String::from("it is tweet content"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", my_tweet.summarize());

    let new_post = BlogPost {
        title: String::from("It is my first blog post"),
        date: String::from("2022-01-26"),
        author: String::from("Shayan"),
        content: String::from("hey it is time to learn rust"),
    };
    println!("New post {}", new_post.summarize());
}
