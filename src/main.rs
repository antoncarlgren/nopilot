use nopilot;

nopilot::generate!("Write a Rust function called 'sqrt', that takes an u32 and returns the square root of that number.");
nopilot::generate!("Write a Rust function called 'to_char_vec' that takes a &str as an argument, and returns a Vec<&str>, where each item represents a character in the provided argument.");

fn main() {
    dbg!(sqrt(144));
    dbg!(to_char_vec("Hello, world!"));
}
