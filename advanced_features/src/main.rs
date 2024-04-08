use std::slice;

fn main() {
    raw_pointers();
    unsafe {
        dangerous(); // must call unsafe fn inside unsafe block {}
    }
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3); // we don't need unsafe when calling split_at_mut b/c it is a safe abstraction over unsafe code
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    print_global_var();
    add_to_count(3);
    unsafe {
        println!("COUNTER: {COUNTER}");
    }

    unsafe_unions();
    test_override_add();
    disambiguating_trait_fn_names();
    supertrait_on_point();
    print_newtype();
    type_aliasing();
    dyn_size_types();
    function_pointer();
    let _closure = returns_closure();
    test_declarative_macro();
    test_procedural_macro();
}

fn raw_pointers() {
    // Dereferencing a raw pointer
    // 1. Can ignore borrowing rules by having both immutable and mutable pointers or multiple mutable pointesr to the same location
    // 2. Aren't guaranted to point to valid memory
    // 3. Are allowed to be null
    // 4. Don't implement any automatic cleanup
    let mut num = 5;
    let r1 = &num as *const i32; // cast immutable ref to raw pointer
    let r2 = &mut num as *mut i32; // cast mutable ref to raw pointer
                                   // the above raw pointesr are valid because we created them from valid references

    unsafe {
        println!("r1 is {}", *r1); // deref to read value at location
        println!("r2 is {}", *r2); // deref to read value at location
    }

    // Below, we create a raw pointer whose validity we're uncertain of
    let address = 0x012345usize; // arbitrary memory location
    let r = address as *mut i32; // r would be undefined. this is fine, we just can't deref in safe code
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) }; // this will likely crash when the slice is used b/c we're slicing 10k items arbitrarily. We aren't sure those locations in memory have valid data.

    // Creating a pointer does no harm; only when we try to access value that it points at that we might up dealing w/ an invalid value
}

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // Although this Fn calls unsafe code, the compiler can guarantee that this fn is safe based on how we've written it
    let len = values.len();
    let ptr = values.as_mut_ptr(); // mutable pointer to slice's buffer | RAW POINTER
    assert!(mid <= len); // This is important to guarantee safety. We know all pointers within unsafe block will be valid b/c mid is a valid location in memory.
    unsafe {
        // unsafe b/c this code has to take a raw pointer
        (
            slice::from_raw_parts_mut(ptr, mid), // starts from beginning of ptr and creates slice of mid items long
            slice::from_raw_parts_mut(ptr.wrapping_add(mid), len - mid), // calculates offset from ptr (of mid items) and creates slice of len - mid items long
        )
    }
}

// We can use `extern` functions to call external code such as from the C standard library
// "C" denotes which ABI (application binary interface) to use. ABI defines how to call fn at assembly level.
extern "C" {
    fn abs(input: i32) -> i32;
}

// We can also write Rust fns to be called from other languages
// Need no_mangle so rust compiler doesn't mangle name for other parts of the program, but are less human readable
// We would be able to call this fn after it's compiled to a shared library and linked from C
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust fn from C!");
}

// Rust global variables are called static variables
// Diffs between constants and static variables
// 1. Static have fixed address in memory, Constants are allowed to duplicate data whenver used
// 2. Static variables can be mutable. Accessing and modifying mutable static variables is UNSAFE
static HELLO_WORLD: &str = "Hello, world!";
fn print_global_var() {
    println!("value is: {HELLO_WORLD}");
}

static mut COUNTER: u32 = 0; // with mutable data that's globally accessible, difficult to ensure no data races. thus, unsafe
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// A trait is unsafe when the trait has at some invariant the compiler can't verify
// Example, if we try to multithread send/sync a raw pointer type, we can't guarantee this data is safe. Therefore, we'll need to mark that raw pointer type as also Send or Sync with UNSAFE
unsafe trait Foo {
    // trait stuff goes here
}
unsafe impl Foo for i32 {
    // method impls go here
}

use std::marker::Send;
use std::ptr::NonNull;

// Custom abstract wrapper around raw pointer NonNull. Always prefer to use Arc and Mutex when possible.
struct MySafePointer<T> {
    ptr: NonNull<T>, // Using NonNull for demonstration; it's like *mut T but non-nullable
}

unsafe impl<T> Send for MySafePointer<T> {}

// We can also use unions which are like structs but only one declared field can be used in an instance at one time.
// This is unsafe because Rust can't guarantee the type of data currently stored in the union instance.
fn unsafe_unions() {
    union IntOrFloat {
        i: u32,
        f: f32,
    }
    let mut u = IntOrFloat { f: 1.0 };
    // Reading the fields of a union is always unsafe
    assert_eq!(unsafe { u.i }, 1065353216);
    // Updating through any of the field will modify all of them
    u.i = 1073741824;
    assert_eq!(unsafe { u.f }, 2.0);
}

// Using `unsafe` code is totally correct and not frowned upon. It's just trickier to get correct. By doing so responsibly, we can leverage these superpowers.

/*
Associated Types
pub trait Iterator {
  type Item; // placeholder trait
}

impl Iterator for Counter{
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> { ... }
}

So why don't we just use a generic declaration to do the above?
B/c w/ generics we have to specify the type everytime we use it. With associated types we only need to define it once b/c you cannot implement a trait on a type multiple times.
pub trait Iterator<T> {
  fn next(&mut self) -> Option<T>
} // this would require Iterator<String> or Iterator<u32> every time we want to use it for example.
*/

// Default Generic Type Parameteres and Operator Overloading
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn test_override_add() {
    let x = Point { x: 1, y: 2 } + Point { x: 1, y: 2 };
    assert_eq!(x, Point { x: 2, y: 4 },);
    println!("{:?}", x);
}

trait Pilot {
    fn fly(&self) {
        println!("flying a plane");
    }
}
trait Wizard {
    fn fly(&self) {
        println!("magic flying");
    }
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("captain speaking");
    }
}
impl Wizard for Human {}

impl Human {
    fn fly(&self) {
        println!("waving arms furiouslu");
    }
}

fn disambiguating_trait_fn_names() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
}

// Supertraits which are traits that depend on other traits as a requirement
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", "*".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", "*".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}
fn supertrait_on_point() {
    let point = Point { x: 1, y: 2 };
    point.outline_print();
}

// Newtype pattern to Implement External Traits
// Use newtype pattern to bypass locality restriction implemtning traits on types
// We need to use a Wrapper<> to house the type we want to implement our trait for

struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // We use self.0 to access the inner Vec of our Wrapper
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn print_newtype() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}

// Advanced Types
// We can use the newtype pattern to abstract implementation details of a type: expose a public API w/ newtype that is different from the private inner type
// E.g. Use People to wrap HashMap(i32, String), and code using People only interacts with the public API we provide.
// In other words, newtypes are a lightweight way to achieve encapsulation.

// Type Aliasing
fn type_aliasing() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
    // This can lead to issues where i32 and KM are mixed up by devs

    // Instead of implementing a type of Box<dyn Fn() + Send + 'static> all over we can instead use an alias like
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let _f: Thunk = Box::new(|| println!("boing"));
    fn _takes_aliased_long_type(_f: Thunk) {}
    // instead of
    fn _takes_long_type(_f: Box<dyn Fn() + Send + 'static>) {}
    // much cleaner!
}

// Another example where aliasing a common lengthy return type cleans up a lot! Use generics on Result and pass it down.
type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&self, fmt: fmt::Arguments) -> Result<()>;
}

// The Never Type that Never Returns
// fn bar() -> ! {}
// `!` can be coerced into any other type. For example, in a match { Err(_) => continue } arm
// Another example is in a loop {}, the loop never ends so Rust determines loop has type of `!`

// Dynamically Sized Types and the Sized Trait
fn dyn_size_types() {
    // Below is invalid b/c str (not &str) is a Dynamically Sized Type (DST), therefore str would have to be same length
    // let s1: str = "hey";
    // let s2: str = "heyo";
    // The way to solve this is using &str which stores TWO values, the start address and the length.
    // Golden Rule of DST: Always put them behind a pointer of some kind

    trait Example: fmt::Debug {
        fn say_hi(&self);
    }
    #[derive(Debug)]
    struct PigHouse<'a> {
        pigs: Vec<Box<&'a dyn Example>>,
    }
    #[derive(Debug)]
    struct GuineaPig {
        name: String,
    }
    impl Example for GuineaPig {
        fn say_hi(&self) {
            println!("hi");
        }
    }
    let gp = GuineaPig {
        name: String::from("Jason"),
    };
    let ex_struct = PigHouse {
        pigs: vec![Box::new(&gp)],
    };
    println!("{:#?}", ex_struct);

    fn generic_known_size_default<T>(t: &T) {} // by default generic fns only work on types w/ known size at compile time
    fn generic_loose_size_override<T: ?Sized>(t: &T) {} // use ?Sized to loosen this requirement
}

// Advanced Functions and Closures
// Function Pointers
fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
fn function_pointer() {
    let ans = do_twice(add_one, 5);
    println!("Answer is {ans}.");
    function_closures();
}
fn function_closures() {
    // Using either a pointe or closure
    let nums = vec![1, 2, 3];
    let _use_closure: Vec<String> = nums.iter().map(|i| i.to_string()).collect();
    let _use_pointer: Vec<String> = nums.iter().map(ToString::to_string).collect();
}
// Returning Closures
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// Macros
// Declarative macros = macro_rule!
// Procedural Macros = #[derive], attribute macros, function macros
// Difference btwn macros and functions
// 1. Macros = writing code that writes other code = Metaprogramming
// 2. Macro code is expanded before compiler interprets code
// 3. Macro downside is, more complex to define because you are writing Rust code to write Rust code
// 4. You must define or bring macros into scope before calling them, whereas fn can be defined and called anywhere

// Declarative Macro
// E.g. vec![] is defined as:
#[macro_export]
macro_rules! sick_vec {
    ( $( $x:expr ),*) => {
        {
          let mut temp_vec = Vec::new();
          $(
            temp_vec.push($x);
          )*
          temp_vec
        }
    };
}

fn test_declarative_macro() {
    let x = sick_vec![1, 2, 3];
    println!("my sick vec {:?}", x);
}

// Custom derive Macro (procedural)
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn test_procedural_macro() {
    Pancakes::hello_macro();
}

// Attribute Macros
// Similar to derive macros, they are defined as procedural macro, but are more flexible. You define them pretty much the same way though...new crate etc
// Can define on more than struct and enums, even functions like below:
// #[route(GET, "/")]
// fn index() {}

// The signature of route macro would be:
// #[proc_macro_derive]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// Function macros
// `let sql = sql!(SELECT * FROM posts WHERE id=1);`
// Looks similar to macro_rules! macros, but are more flexible. Definition signature is similar to custom_derive macro signature:
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {}
