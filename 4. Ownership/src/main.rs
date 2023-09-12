fn main() {
    // ----- Ownership -----
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    let x = 5; // x owns 5
    let y = x; // x is copied to y

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy
    println!("s1 = {}, s2 = {}", s1, s2);


    // ----- Ownership and Functions -----
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
    println!("{}", s); // ... and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
    println!("{}", x); // x is still valid here

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("s1 = {}, s3 = {}", s1, s3);

    // ----- References -----

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 is a reference to s1
    println!("The length of '{}' is {}.", s1, len);

    // ----- Mutable References -----
    let mut s = String::from("hello");
    change(&mut s);

    let word = first_word(&s);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2]; // [2, 3]

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    let length = s.len(); // len() returns the length of a String
    length // length is returned and moves out to the calling function
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // convert string to array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iterate over array of bytes
        if item == b' ' { // if byte is a space
            return &s[0..i]; // return string slice
        }
    }

    &s[..] // return entire string
}