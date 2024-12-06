/*
 * Правило 1
 * Если слово начинается с гласной, или `xr` или `yr`, то добавить `ay` в конец.
 *
 * Правило 2
 * Если слово начинается с одной или более согласных, то переместить их в конец,
 * после добавить `ay`
 *
 * Правило 3
 * Если слово начинается с одной или более согласных, и после них идет `qu`,
 * то переместить их в конец вместе с `qu`, и после добавить `ay`
 *
 * Правило 4
 * Если слово начинается с одной или более согласных, и после них идет `y`,
 * то переместить согласные в конец, и после добавить `ay`
 */

use std::char;

fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn translate_word(word: &str) -> String {
    let mut result = String::new();

    let mut chars = word.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            // Rule 1
            _ if is_vowel(&c)
                | (c == 'x' && chars.peek() == Some(&'r'))
                | (c == 'y' && chars.peek() == Some(&'t')) =>
            {
                result.push_str(word);
                result.push_str("ay");
                break;
            }
            // Rule 2, 3, 4
            _ => {
                let mut consonants = String::new();

                consonants.push(c);

                let mut count = 0;

                while let Some(&c) = chars.peek() {
                    // Rule 3
                    if !(c == 'u' && consonants.ends_with('q')) {
                        // Rule 2
                        if is_vowel(&c) {
                            break;
                        }
                    }

                    // Rule 4
                    if c == 'y' {
                        break;
                    }

                    chars.next();
                    consonants.push(c);
                    count += 1;
                }

                result.push_str(&word[count + 1..]);
                result.push_str(&consonants);
                result.push_str("ay");
                break;
            }
        }
    }

    result
}

pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}
