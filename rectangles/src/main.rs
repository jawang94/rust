#[derive(Debug)]
struct Rect {
    height: u32,
    width: u32,
    color: Color,
}

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Blue,
    None,
}

impl Color {
    fn to_str(&self) -> &str {
        match self {
            Color::Red => "red",
            Color::Blue => "blue",
            Color::None => "",
        }
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

impl Rect {
    fn default_if_none(height: Option<u32>, width: Option<u32>, color: Option<Color>) -> Self {
        let height: u32 = if height != None { height.unwrap() } else { 0 };
        let width: u32 = if width != None { width.unwrap() } else { 0 };
        let color: Color = if color != None {
            color.unwrap()
        } else {
            Color::None
        };

        Self {
            height,
            width,
            color,
        }
    }

    fn square(edge_length: u32) -> Self {
        Self {
            height: edge_length,
            width: edge_length,
            color: Color::None,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, comparable_rect: &Rect) -> bool {
        comparable_rect.area() <= self.area()
    }
}

fn main() {
    let rect1 = Rect::default_if_none(Some(50), Some(30), None);
    let rect2 = Rect::default_if_none(Some(5), Some(3), None);
    let rect3 = Rect::default_if_none(Some(415), Some(312), Some(Color::Blue));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let sq = Rect::square(10);
    println!("Square: {:#?}", sq);

    println!("Color of rect3 is {:?}", rect3.color.to_str());

    let x = IpAddr::V4(String::from("123,0,0,1"));
    let y = IpAddr::V6(String::from("::1"));
    println!("x is {:?}", x);
    println!("y is {:?}", y);

    let a = plus_one_maybe(Some(3)).unwrap();
    println!("yip {:?}", a);
    println!("nop {:?}", plus_one_maybe(None));

    if let 4 = a {
        println!("got em bish");
    }
}

fn plus_one_maybe(x: Option<u32>) -> Option<u32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
        other => Some(69),
    }
}
