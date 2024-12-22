use std::io; 
fn main() { 
    let mut input_f = String::new(); 
    io::stdin().read_line(&mut input_f).expect("norm vvedi"); 
    let fl: f64 = input_f.trim().parse().expect("veshestvennoe"); 
    let cels = (5.0/9.0) * (fl-32.0); 
    println!("{:.2}", cels ); 
}