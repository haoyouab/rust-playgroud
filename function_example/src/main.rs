#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Self {
        Point { x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }
}

fn main() {
    let p1 = Point::origin();
    let p2 = Point::new(2.3, 4.5);

    println!("p1 = {:?}", p1);
    println!("p2 = {:?}", p2);

    let rectangle = Rectangle {
            p1: p1,
            p2: p2,
    };

    println!("rectangle area = {}", rectangle.area());
}