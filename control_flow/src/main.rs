#![allow(unreachable_code)]
#![allow(dead_code)]

fn if_else_flow() {
    let n = 5;

    if n > 5 {
        println!("n > 5");
    } else if n < 5 {
        println!("n < 5");
    } else {
        println!("n = 5");
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, half the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}

#[allow(unused_labels)]
fn loop_flow() {
    'outer: loop {
        println!("Entering outer loop");
        'inner: loop {
            println!("Entering inner loop");
            break 'outer;
        }
    }
    println!("Exiting outer loop");
}

fn for_flow() {
    let mut sum = 0;
    for n in 1..=100 {
        sum += n;
    }
    println!("sum = {}", sum);

    let names = vec!["Bob", "Ferris", "Frank"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
}

fn match_flow() {
    /* tuple */
    let tup = (0, 1, 2);

    match tup {
        (0, x, y) => println!("tuple starts with 0 and 2nd={} 3rd={}", x, y),
        (1, ..) => println!("tuple starts with 1"),
        _ => println!("tuple start with something else"),
    }

    /* enum */
    let color = Color::RGB(12, 32, 56);

    match color {
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::RGB(r, g, b) => println!("RGB color {{{}:{}:{}}}", r, g, b),
        _ => println!("Other"),
    }

    /* de-reference & de-structure */
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match *reference {
        val => println!("Got a value via destructuring: {:?}", val),
    }

    /* bindings */
    let age = 12;

    match age {
        n @ 1..=12 => println!("{} is between 1-12", n),
        n => println!("age is {}", n),
    }

    /* if let */
    let opt = Some(7);

    if let Some(var) = opt {
        println!("Matched {:?}", var);
    }
}

fn main() {
    if_else_flow();
    loop_flow();
    for_flow();
    match_flow();
}
