use std::collections::HashMap;

fn main() {
    // Creating a vector that is empty, requires type declaration
    let v1: Vec<i32> = Vec::new();

    // Rust has a vector macro to create a new one
    let v2 = vec![1, 2, 3];

    // Mutable Vectors
    let mut v3 = vec![4, 5, 6];

    v3.push(7);

    // Referencing values

    // Indexing Syntax

    // Using & and [] gives us a reference to the element at i
    let a: &i32 = &v3[2];
    println!("The third element is {a}");

    // Get method

    let b: Option<&i32> = v3.get(2);
    match b {
        Some(b) => println!("The third element is {b}"),
        None => println!("There is no third element."),
    }

    for i in &mut v3 {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let data = "contents";

    let s = data.to_string();

    let mut other = String::new();

    other.push_str("foo");

    let other2 = "bar";

    other.push_str(other2);

    other.push('h');

    println!("other is {other}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    println!("---------------------------");

    for b in "Зд".bytes() {
        println!("{b}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world hello hello";

    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert returns a mutable reference to the value of the key
        let curWordCount = map2.entry(word).or_insert(0);
        *curWordCount += 1;
    }

}
