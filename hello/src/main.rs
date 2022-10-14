#![warn(clippy::all, clippy::pedantic)] 

fn main() {
    let values = ["One", "Two", "Three"];
    for value in &values {
        println!("{}", value);
    }
}
