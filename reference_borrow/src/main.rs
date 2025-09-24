fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s1={}", s2);
    let string = "hello";
    println!("string={}", string);
    let str_len = calculate_length(string);
    println!("str_len={}", str_len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}