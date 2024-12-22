use std::io;

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn to_pig_latin(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let mut chars: Vec<char> = word.chars().collect();
    if let Some(first) = chars.first() {
        if is_vowel(*first) {
            // For words starting with vowels, add "hay"
            format!("{}-hay", word)
        } else {
            // For words starting with consonants
            let first_char = chars.remove(0);
            let new_word: String = chars.into_iter().collect();
            format!("{}-{}ay", new_word, first_char)
        }
    } else {
        String::new()
    }
}

fn main() {
    println!("Введите текст для преобразования в поросячью латынь:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения строки");

    let result: String = input
        .split_whitespace()
        .map(to_pig_latin)
        .collect::<Vec<String>>()
        .join(" ");

    println!("Результат: {}", result);
}