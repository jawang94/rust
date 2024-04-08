use std::thread;

fn main() {
    closures();
    iterators();
}

fn closures() {
    let closure_add_one = |x| x + 1;
    let imp: i32 = 1;
    let test = closure_add_one(imp).to_string();
    println!("{test}");

    // Borrow immutably
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // Borrow mutably
    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);
    let mut borrows_mutably = || list2.push(4);
    borrows_mutably();
    println!("After calling closure: {:?}", list2);

    // Take ownership
    let list3 = vec![1, 2, 3];
    thread::spawn(move || {
        println!("From thread: {:?}", list3);
    })
    .join()
    .unwrap();

    // Moving out of closures and fn traits (FnOnce, FnMut, Fn)
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let mut list4 = [
        Rectangle {
            width: 32,
            height: 32,
        },
        Rectangle {
            width: 132,
            height: 56,
        },
        Rectangle {
            width: 3,
            height: 31,
        },
    ];
    // let mut sort_operations = vec![];
    // let value = String::from("lolz");
    // list4.sort_by_key(|r| {
    //     sort_operations.push(value); <-- ERROR because attempts to move value out of closure into the vec
    //     r.width
    // });
    let mut sort_operations = 0;
    let value = String::from("lolz");
    list4.sort_by_key(|r| {
        sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {sort_operations} operations", list4)
}

// Note: All iterators are lazy
// Iterators are one of Rust's zero-cost abstractions (similar to C++'s zero-overhead principles by Bjarne Stroustrup). Looks high level, compiles to super efficient assembly.
fn iterators() {
    // `for` loop takes ownership and converts iterator into mut instead
    // .iter() requires mut list, as it changes internal state to track where it is. uses immutable refs
    // .iter_mut() to iterate over mutable refs
    // .into_iter() would transfer ownership to the iterator (e.g. for returning owned values)
    // Methods in std lib that call `next` on Iterator trait *consume* the iterator. They are called consume adapters.
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    // we wouldn't be able to use v1_iter again here because it was consumed by sum() which takes ownership
    println!("{total}");

    // Some methods produce other iterators, Iterator Adapters
    let mapped_v1: Vec<i32> = v1.iter().map(|r| r + 1).collect();
    println!("Mapped {:?}", mapped_v1);

    // You can also capture environment from closures
    struct Shoe {
        size: u32,
        style: String,
    }
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) {
        shoes.into_iter().filter(|x| x.size == shoe_size).collect()
    }
}
