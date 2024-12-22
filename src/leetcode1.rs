use std::io;
fn main() {
    let mut input1 = String::new();    
    let mut input2 = String::new();
    io::stdin().read_line(&mut input1).expect("Ошибка");
    let num1: f64 = input1.trim().parse().expect("Ошибка");
    io::stdin().read_line(&mut input2).expect("Ошибка");
    let num2: f64 = input2.trim().parse().expect("Ошибка");
    let max_value = if num1 > num2 { num1 } else { num2 };
    println!("Из {:.3} и {:.3} больше {:.3}", num1, num2, max_value);
}