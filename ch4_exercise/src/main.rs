// Take a string of words separated by spaces
// Return the first word it finds in that string
fn main() {
    println!("Hello, world!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
