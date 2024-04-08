fn main() {
    // match Arms
    let temp = 0;
    match temp {
        0 => println!("bingo"),
        1 => println!("hmm"),
        _ => println!("bonked"),
    }

    // if let statements
    let age: Result<u8, _> = "34".parse();
    if let Ok(age) = age {
        println!("got some age: {age}");
    }

    // while let loops
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("got the top {top}");
    }

    // for loops
    let v = vec![1, 2, 3];
    for (i, v) in v.iter().enumerate() {
        println!("{i}, {v}");
    }

    // Refutable vs Irrefutable patterns
    // Refutable e.g. if let and while let expressions (program can proceed if no match)
    // Irrefutable e.g. for and let expressions (program cannot do anything if no match)

    // match starts new scope and will shadow variables defined in same scope
    // variables defined inside match statement will not take the value of similarly named variables outside
    // to do this we'd need match guarding.
    let x = Some(5);
    let y = 10;

    match x {
        Some(5) => println!("got 5"),
        Some(y) => println!("{}", y), // this would print 5, because this new y var would bind to the value inside x's Some() which is 5
        _ => println!("default"),
    }

    // Matching multiple patterns is also possible
    match x {
        Some(5) | Some(10) => println!("all good"),
        Some(33) => println!("meh"),
        _ => println!("nothing"),
    }

    // Match ranges with `..=` (INCLUSIVE)
    match y {
        1..=10 => println!("yessir"), // one through ten
        _ => println!("default"),
    }

    let z = 'c'; // remember that '' = char and "" = &str
    match z {
        'a'..='g' => println!("we can also match chars"),
        'h'..='z' => println!("later chars"),
        _ => println!("nothing"),
    }

    // We can destructure to break values apart
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 3 };
    let Point { x: a, y: b } = p; // shorthand let Point { a, b } = p;
    println!("{a}, {b}");
    assert_eq!(0, a);
    assert_eq!(3, b);

    // Match with destructuring
    match p {
        Point { x: 0, y } => println!("point is on x axis ({:?},{y})", x),
        Point { x, y: 0 } => println!("point is on y axis ({x},{y})"),
        Point { x, y } => println!("point is on neither axis ({x},{y})"),
    }

    // Destructure enums and match
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(1, 2, 3);
    match msg {
        Message::Quit => println!("beep"),
        Message::Move { x, y } => println!("{x},{y}"),
        Message::Write(x) => println!("writing: {x}"),
        Message::ChangeColor(r, g, b) => println!("{r},{g},{b}"),
    }

    // Destructure Nested Structs and Enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum UpdatedMessage {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let new_msg = UpdatedMessage::ChangeColor(Color::Hsv(1, 2, 3));
    match new_msg {
        UpdatedMessage::ChangeColor(Color::Rgb(r, g, b)) => println!("rgb {r},{g},{b}"),
        UpdatedMessage::ChangeColor(Color::Hsv(h, s, v)) => println!("rgb {h},{s},{v}"),
        _ => println!("beep"),
    }

    // Destructure Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: 10 });
    println!("{feet}ft {inches}inches at ({x},{y})");

    // Ignoring entire value
    // Using underscore_
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }
    // Ignore parts of a value
    // Using nested underscore_
    let mut settings_value = Some(5);
    let mut new_settings_value = Some(10);
    match (settings_value, new_settings_value) {
        (Some(_), Some(_)) => {
            println!("this is going to match the Some, no matter what's inside of it")
        }
        _ => settings_value = new_settings_value,
    }
    println!("Settings is {:?}", settings_value);

    // Using underscore_ multiple times
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // Using leading underscore_ for unused variable
    let _x = "hey dude";
    println!("never used _x hehe");
    let s = Some(String::from("this is unused too"));
    // if let Some(_s) = s { // doesn't work because s doesn't get moved into the if let scope
    //     println!("found a string!")
    // }
    // This does work
    if let Some(_) = s {
        println!("found a string!")
    }

    // Match remaining parts with ..
    // Note .. can only be used once per tuple or vec pattern
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("first: {first} and last: {last}"),
    }

    // Extra conditionals with match guards
    // Similar to Python3 in-line if-else statements
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("even number boy: {x}"),
        Some(x) => println!("odd number boy: {x}"),
        None => (),
    }

    let x = 4;
    let y = false;
    match x {
        (4 | 5 | 6) if y => println!("yes"), // doesn't match b/c if statemenet applies to the entire pattern
        _ => println!("no"),
    }

    // @ Bindings allow us to create a cariable that holds a value at the same we're testing that value in pattern matching
    enum AnotherMessage {
        Hello { id: i32 },
    }
    let another_msg = AnotherMessage::Hello { id: 1 };
    match another_msg {
        AnotherMessage::Hello {
            id: id_variable @ 0..=3, // bind id_variable to matched id within range 0 to 3
        } => println!("Found id in range: {id_variable}"),
        AnotherMessage::Hello { id: 4..=12 } => println!("found id in diff range"),
        AnotherMessage::Hello { id: _ } => println!("any other id"),
    }
}
