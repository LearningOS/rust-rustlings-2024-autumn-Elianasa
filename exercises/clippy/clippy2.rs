// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a

fn main() {
    let mut res = 42;
    let option = Some(12);
    while let Some(x) =option {//for循环不可靠
        res += x;
    }
    println!("{}", res);
}
