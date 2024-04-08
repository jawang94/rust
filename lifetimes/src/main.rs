use std::fmt::Display;

#[derive(Debug)]
struct LifeStruct<'a, GenericStr: AsRef<str> + ?Sized> {
    borrowed_something: &'a GenericStr, // borrowed str w/ bound lifetime
    owned_something: String,            // borrowed str w/o bound lifetime
}

fn main() {
    let string1 = String::from("yobose");

    let _yeep = LifeStruct {
        borrowed_something: "borrowed beesh",
        owned_something: String::from("owned bish"),
    };

    {
        let string2 = String::from("xyz");
        let not_a_string = 1;
        let result = longest(string1.as_str(), string2.as_str());
        let _beep = longest(string1.as_str(), &not_a_string.to_string());
        println!("kekeke bro: {result}");
    }
    let string2 = String::from("i am here!!");
    longest_with_an_announcement(&string1, &string2, "Wakanda 4 eva");
}

fn longest<'a, T: AsRef<str> + ?Sized>(str1: &'a T, str2: &'a T) -> &'a T {
    if str1.as_ref().len() > str2.as_ref().len() {
        str1
    } else {
        str2
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// For advanced usage check https://doc.rust-lang.org/reference/trait-bounds.html
