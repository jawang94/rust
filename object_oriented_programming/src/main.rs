fn main() {
    screen_demo();
    blog_it_up();
}

// `Pub` allows for encapsulation of implementation details
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Inheritance: Rust has no way for structs to inherit parent struct's field. However, traits simulates this behavior.
// Through traits, we can also override fault implementations of a method.

// Polymorphism: Allows typing a child to be used in the same places as a parent.
// Rust uses generics to impose constraints on possible types and trait bounds, which is similar.
// This is called "bounded parametric polymorphism"
// Inheritance has fallen out of favor as a programming design solution b/c of risk of sharing more than necessary (coupling)

use object_oriented_programming::gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to draw select box
    }
}

fn screen_demo() {
    use object_oriented_programming::gui::{Button, Screen};

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// Now let's look at how we can emulate State, another key OOP concept

use object_oriented_programming::blog::Post;
fn blog_it_up() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch", post.content());
}
