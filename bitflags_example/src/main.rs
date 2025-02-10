use bitflags::{bitflags, Flags};
use bitflags_derive::FlagsDisplay;

bitflags! {
    #[derive(Debug, FlagsDisplay)]
    struct MyFlags: u8 {
        const A = 1;
        const B = 1 << 1;
        const C = 1 << 2;
    }
}

fn defined_flags<F: Flags>() -> usize {
    F::FLAGS.iter().filter(|f| f.is_named()).count()
}

fn main() {
    println!("Named Flags {}", defined_flags::<MyFlags>());

    let var = MyFlags::A.bits();
    let from_retain = MyFlags::from_bits_retain(0b1000_0001);
    let from = MyFlags::from_bits(0b1000_0000);

    let empty_flags = MyFlags::empty();
    let all_flags = MyFlags::all();
    let from_contain = from_retain.contains_unknown_bits();
    let empty_contain = empty_flags.contains_unknown_bits();
    let all_contain = all_flags.contains_unknown_bits();

    println!("var = {:?}", var);
    println!("from_retain = {:?}", from_retain);
    println!("empty_flags = {:?}", empty_flags);
    println!("all_flags = {:?}", all_flags);
    println!("from_retain contains unknown bits = {:?}", from_contain);
    println!("empty_flags contains unknown bits = {:?}", empty_contain);
    println!("all_flags contains unknown bits = {:?}", all_contain);

    if let Some(f) = from {
        println!("from {}", f);
    } else {
        println!("from None");
    }

    let name = MyFlags::from_name("C");
    if let Some(n) = name {
        println!("Flags from name is {}", n);
    } else {
        println!("No flags from name");
    }

    let invalid_name = MyFlags::from_name("K");
    if let Some(n) = invalid_name {
        println!("Flags from name is {}", n);
    } else {
        println!("No flags from name");
    }

    let iter_var = from_retain.iter();
    for i in iter_var {
        println!("{}", i);
    }
}
