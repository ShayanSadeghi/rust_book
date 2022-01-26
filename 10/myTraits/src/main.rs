// section 10.2
use myTraits::{Summary, Tweet}; //we should import Summary too
fn main() {
    let my_tweet = Tweet {
        username: String::from("some_un"),
        content: String::from("it is tweet content"),
        reply: false,
        retweet: false,
    };
    print!("1 new tweet: {}", my_tweet.summarize())
}
