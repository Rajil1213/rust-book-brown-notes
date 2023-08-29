struct Post {
    content: String,
}

struct DraftPost {
    content: String,
}

struct PendingReviewPost {
    content: String,
}

impl Post {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
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
        // assert_eq!("", post.content());

        let pending_review_post = post.request_review();
        // assert_eq!("", post.content());

        let approved_post = pending_review_post.approve();
        assert_eq!(content, approved_post.content());
    }
}
