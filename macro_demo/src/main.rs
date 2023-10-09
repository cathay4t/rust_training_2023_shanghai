// Based on https://doc.rust-lang.org/rust-by-example/macros/dsl.html

macro_rules! calculate {
    // available designators:
    // https://doc.rust-lang.org/rust-by-example/macros/designators.html
    ($prefix:literal, $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{}: {} = {}", $prefix, stringify!{$e}, val);
        }
    };
}

fn main() {
    calculate! {
        "Quiz 1", 1 + 2
    }

    calculate! {
        "Quiz 2", (1 + 2) * (3 / 4)
    }
}
