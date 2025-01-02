use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
struct Structure(i32);

#[derive(Debug)]
#[allow(dead_code)]
struct Deep(Structure);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x={}, y={}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    // Some comments here
    println!("Hello world!");

    /*
    some more comments here
    some other comment here
    blah blah blah
     */
    println!("I'm a Rustacean!");

    println!("{} days", 31.4);
    println!("{0:b} {1} {2} {0}", 10, 2, 3);
    println!("Today is the day {day} of the year {year}", day=3, year=2025);

    let day2: i8 = 1;
    let year2: i32 = 2024;
    println!("Today is the day {day2} of the year {year2}");

    println!("My name is {0}, {1} {0}", "Bond", "James");

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    println!("Now {:?} will print!", Structure(3));
    println!("Go Deep {:#?}", Deep(Structure(7)));

    println!("This is the Point: {}", Point2D{x: 3.3, y: 5.5});

    let complex = Complex{real: 3.3, imag: 7.2};
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    let v = List(vec![5, 6, 8]);
    println!("{}", v);
    let v = List(vec!());
    println!("{}", v);

}
