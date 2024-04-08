use traits::aggregator::{notify as aggregator_notify, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Jason"),
        content: String::from("da warudo bruv, let's gooo yeee"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("yeeee its crispy");
    aggregator_notify(&tweet);
}
