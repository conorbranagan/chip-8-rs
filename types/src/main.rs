#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// unit struct
#[derive(Debug)]
struct Unit;

// tuple struct 
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Point { x: x1, y: y1 } = rect.top_left;
    let Point { x: x2, y: y2} = rect.bottom_right;
    (y2-y1).abs() * (x2-x1).abs()
}

fn square(p: &Point, v: f32) -> Rectangle {
    Rectangle{
        top_left: Point{ ..*p },
        bottom_right: Point{
            x: p.x + v,
            y: p.y + v,
        }
    }
}

fn do_structs() {
    let someone: Person = Person{ name: String::from("john"), age: 32};
    
    let name = String::from("name2");
    let age = 27;
    let another_one = Person { name, age };
    println!("The people: {:?} {:?}", someone, another_one);

    let point = Point{ x: 3.5, y: 7.2};
    let another_point = Point{ x: 5.3, y: 1.3};

    println!("point coords: ({} {})", point.x, point.y);

    let bottom_right = Point{ x: 10.3, ..another_point};
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;
    let rect = Rectangle{
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };

    let unit = Unit;
    println!("A UNIT: {:?}", unit);

    println!("Rect: {:?}", rect);
    println!("Area: {}", rect_area(rect));

    let len: f32 = 20.0;
    let sq_p = Point{x: 10.1, y: 10.1};
    let sq = square(&sq_p, len);

    println!("Square: {:?}", sq);
    println!("Square area: {:?} (should be {})", rect_area(sq), len*len);

}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64},
}

impl WebEvent {
    fn inspect(&self) {
        match self {
            Self::PageLoad => println!("page loaded"),
            Self::PageUnload => println!("page unloaded"),
            Self::KeyPress(c) => println!("pressed '{}'.", c),
            Self::Paste(s) => println!("pasted \"{}\"", s),
            Self::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

enum Val {
    First,
    Second,
    Third,
}

enum Color  {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff, 
}

fn do_enums() {
    let pressed = WebEvent::KeyPress('f');
    let pasted = WebEvent::Paste(String::from("hi"));

    // how does borrowing work?
    //inspect(pressed);
    //inspect(pasted);
    pressed.inspect();
    pasted.inspect();

    // putting the enums in scope - cooL!
    use crate::Stage::{Beginner, Advanced};
    use crate::Role::*;

    let stage = Beginner;
    let role = Teacher;

    match stage {
        Beginner => println!("beginner stage"),
        Advanced => println!("pros are here"),
    }

    match role {
        Student => println!("learning"),
        Teacher => println!("teaching"),
    }

    println!("first is {}", Val::First as i32);
    println!("second is {}", Val::Second as i32);
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}


fn main() {
    println!("\n\nStructs");
    do_structs();

    println!("\n\nEnums!");
    do_enums();
}
