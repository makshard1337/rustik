use std::io;

fn main() {
    let mut input = String::new();
    
    println!("vvedi chislo n:");
    io::stdin().read_line(&mut input).expect("vvedi norm chislo");
    let n: usize = input.trim().parse().expect("chislo");

    let fib_number = fibonacci(n);
    println!("{} - chislo fibonachi: {}", n, fib_number);
}

fn fibonacci(n: usize) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            let mut c;
            for _ in 2..=n {
                c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}