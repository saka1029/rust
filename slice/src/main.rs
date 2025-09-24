fn main() {
    let s = String::from("こん𩸽ちは");
    let kon = &s[0..3];
    println!("kon={}", kon);
    println!("chars={:?}", s.char_indices().collect::<Vec<(usize, char)>>());
}
