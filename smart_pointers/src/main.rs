// Choosing between Box<T>, Rc<T>, or RefCell<T>
// 1. Rc<T> enables multiple owners of same data; Box<T> and RefCell<T> have single owners
// 2. Box<T> allows immutable or mutable borrows at compile. Rc<T> allows immutable borrows at compile.
// RefCell<T> allows immutable or mutable borrows at compile.
// 3. Because RefCell<T> allows mutable borrows at runtime, you can mutate the value inside RefCell<T> even when
// RefCell<T> is immutable (UNSAFE)

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    use crate::List::{Cons, Nil};
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Deref w/ standard box
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // allows us to follow y to the actual value its pointing at

    my_box();
    deref_coercion();
    drop_trait();
    ref_counting();
    rc_plus_refcell();
    ref_cycle();
    tree_struct();
}

use std::ops::Deref;
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T; // Associated type, slightly diff way to declare a generic
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn my_box() {
    // Deref w/ custom box
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // allows us to follow y to the actual value its pointing at
}

// Deref Coercion
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn deref_coercion() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // rust is able to deref String to str by calling String::deref automatically
}

// Drop Trait

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn drop_trait() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // We'll get a double free error if we explicitly call a destructor (like c.drop();)
    // However, we can forcibly drop the memory by calling std::mem::drop(<what we want to drop>)
    // How this works is, std::mem::drop takes ownership of the element and schedules Drop::drop to be called at the end of std::mem::drop's scope (aka immediately)
    println!("CustomSmartPointers created.");
}

// Reference counting Rc<T> (e.g. used to track nodes in a graph, which is owned by all edges connected to it). A node shouldn't be cleaned until no more nodes are pointing to it.

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}
use std::rc::Rc;
fn ref_counting() {
    use crate::RcList::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// RefCell<T> and Interior Mutability Pattern
// Interior mutability allows mutating data even w/ immutable refs to it (UNSAFE)
// Both Rc and RefCell are only for use in single-threaded applications
pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent, you're at 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning, you're at 75% of your quota!");
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![].into(),
            }
        }
    }
    impl Messenger for MockMessenger {
        // fn send(&self, message: &str) {
        //     self.sent_messages.push(String::from(message)); // &self is immutable ref so this fails w/o RefCell
        // }
        // &self has to be immutable borrow to meet Messenger trait def
        // NOTE: There can only be one mutable borrow of an element per scope
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message)); // mutable borrow
                                                                         // A borrow error panics at runtime, rather than popping a compile error.
                                                                         // This means discovering bugs later on in dev process: possibly not until it has reached prod
                                                                         // Mutable borrowing is risky, but not any riskier than non-compiled languages
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// Using RefCell<T> in combination with Rc<T>

#[derive(Debug)]
enum MultipleOwnersMutableList {
    Cons(Rc<RefCell<i32>>, Rc<MultipleOwnersMutableList>),
    Nil,
}
use std::cell::RefCell;
fn rc_plus_refcell() {
    use crate::MultipleOwnersMutableList::{Cons, Nil};
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// Reference Cycle Example
#[derive(Debug)]
enum RefCycleList {
    Cons(i32, RefCell<Rc<RefCycleList>>),
    Nil,
}

impl RefCycleList {
    fn tail(&self) -> Option<&RefCell<Rc<RefCycleList>>> {
        use crate::RefCycleList::{Cons, Nil};
        match self {
            Cons(_, next) => Some(next), // if self is self, return next
            Nil => None,
        }
    }
}

// Specifically, this cycles because ref counts for a and b are both 2, and dropping either only lowers count to 1.
// Since they point at each other forever, this memory on heap is never collected unless we do something about it
fn ref_cycle() {
    use crate::RefCycleList::{Cons, Nil};

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // a.tail is set to Nil by default
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // here, we force set a tail == b. therefore a -> b -> a is a cycle
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // Uncomment next line to overflow stack due to cycle
    // This specifically happens because the Debug trait tries to unpack nested structed via DFS
    // This overflows our stack because it can never unnest a->b->a
    // println!("a next item = {:?}", a.tail());
}

// Non-ownership Graph example (parent and child nodes)
// We can use Weak<T> to prevent ref cycles
// Calling `downgrade` on Rc makes it Weak which does not increase strong count, rather weak count
// Weak counts do not need to be 0 for data to be cleaned up
// To access a Weak ref value, you have to `upgrade` an Rc to make sure it still exists
// `upgrade` will return an Option<Rc<T>> which either resolves Some or None
use std::rc::Weak;
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
fn tree_struct() {
    let child_leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&child_leaf),
        Rc::weak_count(&child_leaf)
    );
    println!("leaf.parent = {:?}", child_leaf.parent.borrow().upgrade()); // without upgrading, it may return a stale value
    {
        let parent = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&child_leaf)]),
        });
        // access the leaf's parent field, borrow the mut ref and deref to access Weak from RefCell
        // assign it to the downgraded version of parent
        *child_leaf.parent.borrow_mut() = Rc::downgrade(&parent);
        println!(
            "parent strong = {}, weak = {}",
            Rc::strong_count(&parent),
            Rc::weak_count(&parent)
        );
        println!(
            "child_leaf strong = {}, weak = {}",
            Rc::strong_count(&child_leaf),
            Rc::weak_count(&child_leaf)
        );
        println!(
            "INSCOPE leaf.parent = {:?}",
            child_leaf.parent.borrow().upgrade()
        );
    } // once we leave scope, parent is dropped and the weak pointer on child will now show None when we upgrade it
    println!(
        "OUTOFSCOPE leaf.parent = {:?}",
        child_leaf.parent.borrow().upgrade()
    );
    println!(
        "child_leaf strong = {}, weak = {}",
        Rc::strong_count(&child_leaf),
        Rc::weak_count(&child_leaf)
    );
}
