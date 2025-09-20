fn main() {
    let s = String::from("hello");
    let s1 = takes_and_givs_ownership(s);
    println!("{}", s1);
    let x = 5;
    makes_copy(x);
}

fn takes_and_givs_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}