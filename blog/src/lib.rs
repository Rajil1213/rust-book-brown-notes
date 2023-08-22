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
        self.content.push_str(text);
    }

    pub fn content() -> &str {
        ""
    }
}

trait State {}
struct Draft {}

impl State for Draft {}

#[cfg(test)]
mod blog {
    use super::Post;

    #[test]
    fn only_allows_approved_content_to_be_posted() {
        let mut post = Post::new();
        let content = "I ate a salad for lunch today";

        post.add_text(content);
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!(content, post.content());
    }
}
