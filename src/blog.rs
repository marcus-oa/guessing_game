//-------------------------------------------------------------------------------------------
//---------------- ch17-03-design-patterns ---------------------
//-------------------------------------------------------------------------------------------

// Implementation of the state pattern via a blog posting crate

// initial post struct to hold state and content
pub struct Post {
    // Used so that we have a dynamic state that can be
    // anything that implements State
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // Creates a new instance of Post
    // with a default state of Draft (implements State)
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // uses a mutable instance of 'self' to update the content
    // with a passed in ref to a str
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // this fn takes the state, gets a ref which will be an Option<Box<dyn State>>,
    // unwraps which we know will never be None due to our implementation of the
    // types that implement the trait State and uses the contents method
    // of said type, passing in self such that we can update the contents
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // Updates state of a post with PendingReview
    pub fn request_review(&mut self) {
        // this is why we use an Option, the fn take will set the value to None
        // of the Option<Box<dyn State>> and we instantly reset the state
        // by calling the state types internal implementation of request_review
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    // Same implementation as above where we set he state to None and instantly
    // reset the start to approved (in the fn case)
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// The State trait
// other structs implement this strait and
// allow for the Post struct to hold its dynamic state of anything
// that implements this
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // default implementation so other structs that implement
    // State don't need to specify an implementation
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
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
}