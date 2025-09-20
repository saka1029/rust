fn main() {
    let hello = "hello";
    let mut s = String::from(hello);
    s.push_str(", world!");
    println!("{}", s);    
    let mut s1 = String::from("hello");
    let mut s2 = s1;
    s2.push_str(", world!");
    println!("{}", s2);
    s1 = s2;    // 所有権もどす
    println!("{}", s1);
    let s3 = s1.clone();
    println!("s1={}, s2={}", s1, s3);
}
