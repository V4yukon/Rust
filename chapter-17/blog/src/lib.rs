trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approval(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str{
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
}



//Post
//
//
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
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approval(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approval());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

//Draft
//
//
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {} )
    }

    fn approval(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}


//Pending review
//
//
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approval(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) ->Box<dyn State> {
        Box::new(Draft{} )
    }
}


//Published
//
//
pub struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approval(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft{} )
    }
}




