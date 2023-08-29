pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.state
            .as_ref()
            .unwrap()
            .add_text(text, &mut self.content)
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn add_text<'a>(&self, _text: &'a str, _content: &'a mut String) {}
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
}
struct Draft {}

impl State for Draft {
    fn add_text<'a>(&self, text: &'a str, content: &'a mut String) {
        content.push_str(text);
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approvals: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {
    approvals: u8,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // if already one approval is present, this is the second one
        // so publish
        if self.approvals == 1 {
            return Box::new(Published {});
        }

        Box::new(PendingReview {
            approvals: self.approvals + 1,
        })
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

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

        post.reject();
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!(content, post.content());
    }

    #[test]
    fn only_allow_adding_text_to_draft() {
        let mut post = Post::new();
        let content = "I ate a salad for lunch today";
        let content_to_add = " and I loved it";

        post.add_text(content);
        post.request_review();
        // add text again
        post.add_text(content_to_add);

        post.approve();
        // add text again
        post.add_text(content_to_add);
        post.approve();

        assert_eq!(content, post.content());
    }
}
