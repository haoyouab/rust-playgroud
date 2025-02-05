#[cfg(target_os = "linux")]
fn hello_from_os() {
    println!("Hello from linux");
}

#[cfg(not(target_os = "linux"))]
fn hello_from_os() {
    println!("Hello from other OSes");
}

/* using cfg_if */
cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        fn hello_from_os_if() {
            println!("Hello from linux");
        }
    } else {
        fn hello_from_os_if() {
            println!("Hello from linux");
        }
    }
}

fn main() {
    hello_from_os();

    println!("Are you sure?");

    if cfg!(target_os = "linux") {
        println!("Definitely from linux");
    } else {
        println!("Definitely not from linux");
    }

    hello_from_os_if();
}
