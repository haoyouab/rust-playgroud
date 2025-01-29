macro_rules! aa {
    ($($a:tt)*) => {
        println!("{}", stringify!($($a)*));
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Calling {}", stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    ($x:expr) => {
        println!("{} = {:?}", stringify!($x), $x);
    };
}
fn main() {
    aa!(Fuck);
    aa!(Fuck you);
    println!("Hello, world!");

    create_function!(hello);
    create_function!(world);

    hello();
    world();

    print_result!(1 + 2);
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
