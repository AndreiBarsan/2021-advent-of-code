fn capitalize(s: &str) -> String {
    let result = s.chars().nth(0).unwrap().to_uppercase().collect::<String>() + &s[1..];
    result

}

#[derive(Debug)]
struct Base(i32);

#[derive(Debug)]
struct Nested(Base);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // Get username from environment
    let user = match std::env::var("USER") {
        Ok(user) => user,
        Err(_) => "nobody".to_string(),
    };

    println!("Hello, {name}. I am welcoming you to Rust, {name}!", name=capitalize(&user));
    println!("{number:0>10}", number=42);
    println!("{number: >10}", number=42);
    println!("{number:X>10}", number=42);
    println!("{:💩>10}", 42);

    let pi = 3.141592653589793;
    println!("{:.3}", pi);
    println!("{:.4}", pi);

    let peter = Person { name: "Peter", age:31 };
    println!("{:?} months in a year.", 12);
    println!("{:?}", Base(42));
    // '#?' does a pretty-debug-print
    println!("{:#?}", Nested(Base(42)));
    println!("{:#?}", peter);
}