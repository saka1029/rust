fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
    // reuse
    let spaces = "    ";
    println!("The value of spaces is: ({})", spaces);
    let spaces = spaces.len();
    println!("The value of spaces is: ({})", spaces);

    let x = 2.0;
    let y : f32 = 3.0;

    // adition
    let sum = 5 + 10;
    println!("sum = {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference = {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("product = {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("quotient = {}", quotient);
    let floored = 2 / 3;
    println!("floored = {}", floored);
    let remainder = 43 % 5;
    println!("remainder = {}", remainder);
    let t = true;
    // boolean
    println!("t = {}", t);
    let f: bool = false;
    println!("f = {}", f);
    // char
    let c = 'z';
    println!("c = {}", c);
    let z = 'Z';
    println!("z = {}", z);
    let heart_eyed_cat = '😻'; // utf32=0x0001F63B, utf8=0xF0 0x9F 0x98 0xBB 
    println!("heart_eyed_cat = {}", heart_eyed_cat);
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4,1);
    println!("tup = {:?}", tup); // フォーマット文字列"{}"は使えない
    let (xx, yy, zz) = tup;
    println!("xx={}, yy={}, zz={}", xx, yy, zz);
    let tup0 = tup.0;
    let tup1 = tup.1;
    let tup2 = tup.2;
    println!("tup0={}, tup1={}, tup2={}", tup0, tup1, tup2);
}
