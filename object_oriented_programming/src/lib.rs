pub mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>, // list of any type that implements Draw trait
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    // We could also have defined the struct using generic type params <T>, however that would require all
    // elements in components to be homogenous (aka all Button type or all TextField). Traits allow multiple types.
    // If we know our data will be homogenous, we should use generic type b/c rust will monomorphize definitions at compile time to use concrete types.
    // Monomorphizing allows for performance improvements by Rust via static dispatch
    // Traits on the other hand, require dynamic dispatch which incurs some cost BUT we get extra flexibility

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    impl Draw for Button {
        fn draw(&self) {
            // code to draw the button
        }
    }
}

pub mod blog {
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;
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
        fn reject(self: Box<Self>) -> Box<dyn State> {
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
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

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
            // `.take()` moves state's value out of Post
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }
    }
}

// We could also have used an enum solution to implement blogs
// However, with an enum we need a match statement everytime to handle every possible variant.
// This can result in A LOT of repetitive code, compared to the trait solution

// Traits and structs have their own pitfalls, invalid calls can be made,
// and shared state can result in duplicative code. We can also consider splitting Post
// up into multiple x_Post structs like DraftPost, ApprovedPost, etc. This is not pure OOP.
