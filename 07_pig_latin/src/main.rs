fn main() {
    let phrase = String::from("This is a sentence that will be written in pig latin");
    let phrase_pig_latin = convert_to_pig_latin(&phrase);
    println!("English: {}", phrase);
    println!("Pig latin: {}", phrase_pig_latin);
}

fn convert_to_pig_latin(phrase: &String) -> String {
    let split_phrase: Vec<&str> = phrase[..].split_whitespace().collect();
    let mut translation = String::new();

    for word in split_phrase {
        let latin_word = add_suffix_to_word(&word);
        translation.push(' ');
        translation.push_str(&latin_word);
    }
    translation
}

fn add_suffix_to_word(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut latin = String::from(word);

    let mut first = true;
    let mut starts_with_vowel = false;
    let mut first_letter = ' ';

    // use the String.starts_with() function to improve this
    for letter in latin.chars() {
        if first == false {
            continue;
        }

        first_letter = letter;
        for vowel in vowels.iter() {
            if *vowel == letter {
                starts_with_vowel = true;
            }
        }
        first = false;
    }

    if starts_with_vowel == true {
        latin.push_str("-hay");
    } else {
        latin = String::from(latin.get(1..).unwrap());
        latin = format!("{}-{}{}", latin, first_letter.to_lowercase(), "ay");
    }
    latin
}
