use fix_fn::fix_fn;

fn main() {
    let fact = fix_fn!(|fact, x: u32| -> u32 {
        if x == 0 {
            1
        } else {
            x * fact(x - 1)
        }
    });
    for i in 0..10 {
        println!("fact({}) = {}", i, fact(i));
    }
}
