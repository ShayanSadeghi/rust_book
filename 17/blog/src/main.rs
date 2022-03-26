// Using state pattern as an object-oriented design pattern

use blog::Post;

fn main() {
    let mut post = Post::new(); // the post can be in on of these states: draft, waiting for review and published
                                //the state changing will be managed internally within the Post type

    // save a draft -- not posted yet
    post.add_text("I ate sandwich for lunch today");
    assert_eq!("", post.content());

    // review draft to be able to post it -- not posted yet
    post.request_review();
    assert_eq!("", post.content());

    // post draft after reviewing -- posted now
    post.approve();
    assert_eq!("I ate sandwich for lunch today", post.content());
}
