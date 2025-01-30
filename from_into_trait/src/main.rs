use std::convert::From;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct Number {
    _value: i32,
}

struct Circle {
    radius: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { _value: item }
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let value = 8;
    println!("{:?} from {}", Number::from(value), value);

    let num: Number = value.into();
    println!("{} into {:?}", value, num);

    let cir = Circle { radius: 6 };
    println!("{}", cir.to_string());
}
