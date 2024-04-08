pub mod aggregator {
    pub use core::fmt::Debug;

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub trait Display {}

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn test_fn<T, U>(t: &T, u: &U, switch: bool) -> Box<dyn Summary>
    where
        T: Display + Clone + Summary,
        U: Clone + Debug,
    {
        if switch {
            Box::new(Tweet {
                username: String::from("Testing performance"),
                content: String::new(),
                reply: false,
                retweet: false,
            })
        } else {
            Box::new(NewsArticle {
                headline: String::new(),
                location: String::new(),
                author: String::new(),
                content: String::new(),
            })
        }
    }
}
