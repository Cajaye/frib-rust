fn main() {
    println!("Hello, world!");
    let fib = fib_num(15, 1, 1);
    println!("{}", fib);
}

fn fib_num(nth: usize, mut a: usize, mut b: usize) -> usize {
    loop {
        if b > nth {
            break b;
        }
        let c = a + b;
        a = b;
        b = c;
    }
}
