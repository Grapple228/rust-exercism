use std::collections::HashSet;

pub fn is_anagram(word1: &str, word2: &str) -> bool {
    let word1 = word1.to_lowercase();
    let word2 = word2.to_lowercase();

    word1 != word2 && {
        let mut vec1: Vec<char> = word1.chars().collect();
        let mut vec2: Vec<char> = word2.chars().collect();

        vec1.sort();
        vec2.sort();

        vec1 == vec2
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    for possible_anagram in possible_anagrams {
        if is_anagram(word, possible_anagram) {
            result.insert(possible_anagram.to_owned());
        }
    }

    result
}
