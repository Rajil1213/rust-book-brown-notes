struct Post {
    content: String,
}

struct DraftPost {
    content: String,
}

struct PendingReviewPost {
    approvals: u8,
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
            approvals: 0,
            content: self.content,
        }
    }
}

enum MaybeApproved {
    PendingApproval(PendingReviewPost),
    Approved(Post),
}

impl PendingReviewPost {
    pub fn approve(self) -> MaybeApproved {
        if self.approvals < 1 {
            let pending_approval = PendingReviewPost {
                approvals: self.approvals + 1,
                content: self.content,
            };

            return MaybeApproved::PendingApproval(pending_approval);
        }

        let post = Post {
            content: self.content,
        };

        MaybeApproved::Approved(post)
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod blog {
    use crate::MaybeApproved;

    use super::Post;

    #[test]
    fn only_allows_approved_content_to_be_posted() {
        let mut post = Post::new();
        let content = "I ate a salad for lunch today";

        post.add_text(content);
        // assert_eq!("", post.content());

        let post = post.request_review();
        // assert_eq!("", post.content());

        let post = post.reject();
        // assert_eq!("", post.content());

        let post = post.request_review();

        let post = post.approve();
        let post = match post {
            MaybeApproved::PendingApproval(pending) => pending.approve(),
            MaybeApproved::Approved(_) => {
                panic!("post needs two approvals to be approved");
            }
        };

        let post = match post {
            MaybeApproved::PendingApproval(_) => {
                panic!("post needs to be approved after two approvals")
            }
            MaybeApproved::Approved(approved) => approved,
        };

        assert_eq!(content, post.content());
    }
}
