fn apply<F>(f: F) where
    F: FnOnce() {
        f();
}

fn main() {
    /* closure */
    let one = || 1;
    let closure_annotated = |x: u32| -> u32 { x + 1};

    println!("closure: one -> {}", one());
    println!("closure: closure_annotated -> {}", closure_annotated(5));

    let color = String::from("green");

    let print = || println!("color is {}", color);
    print();

    let ref _reborrow = color;
    print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count = {}", count);
    };
    inc();

    use std::mem;
    let greeting = "hello";
    let mut farewell = greeting.to_owned();
    let diary = || {
        println!("{}", greeting);
        farewell.push_str("!");

        mem::drop(farewell);
    };

    apply(diary);

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1 {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2 {}", vec2.into_iter().any(|x| x == 2));
}