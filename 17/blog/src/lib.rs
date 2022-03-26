pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text); // adding text to content does not related to the 'state' of the Post
    }
    pub fn content(&self) -> &str {
        "" // it's a placeholder
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
