fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1={}", s1);
    println!("s3={}", s3);
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    // println!("s1={}", s1);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}