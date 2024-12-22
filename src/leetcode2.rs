
use std::collections::HashMap;
fn sr_value(numbers: &Vec<i32>) -> f64{
    let ln = numbers.len();
    let mut sum = 0.0 as f64;
    for &num in numbers{
        sum += num as f64;
    }
    let mean = sum as f64 / ln as f64;
    return mean
}

fn medians(numbers: &Vec<i32>) -> f64{
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();
    let ln = &numbers.len();
    let median: f64;
    if ln % 2 == 0{
        median = ((sorted_numbers[ln/2 - 1] + sorted_numbers[ln/2])) as f64 / 2.0;
    } else{
        median = sorted_numbers[ln/2] as f64;
    }
    return median;

}


fn find_mode(numbers: &[i32]) -> Option<i32> {
    let mut frequency: HashMap<i32, i32> = HashMap::new();

    // Подсчет частоты каждого числа
    for &number in numbers {
        *frequency.entry(number).or_insert(0) += 1;
    }

    // Поиск числа с максимальной частотой
    frequency.into_iter().max_by_key(|&(_, count)| count).map(|(number, _)| number)
}


fn main() {
    let numbers = vec![1, 2, 2, 3, 4, 5, 5, 5, 6];
    let sr = sr_value(&numbers);
    let median = medians(&numbers);


    println!("Среднее значение: {:?}", sr);
    println!("Медиана {:?}", median);

    match find_mode(&numbers) {
        Some(mode) => println!("Мода: {}", mode),
        None => println!("Мода не найдена"),
    }
}