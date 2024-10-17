// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!()//宏里写分号了  !标志宏
}
