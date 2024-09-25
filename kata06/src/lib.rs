use std::collections::HashMap;

/// Pair up all anagrams from a word list
pub fn get_anagrams(filepath: &str) -> HashMap<String, Vec<String>> {
    // init a set, each entry is:
    //     ordered version of all the chars in a word (e.g. word -> dorw)
    //     all the anagrams of this sequence (e.g. dorw -> [word, drow, ...])
    let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();

    // read the file
    //     strip away anything non-alphabetical, force lowercase
    //     order the chars
    //     append value to its key
    let file_content = std::fs::read_to_string(filepath).expect("Failed to read file");
    for line in file_content.lines() {
        let cleaned_word: String = line
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>()
            .to_lowercase();
        let mut sorted_chars: Vec<char> = cleaned_word.chars().collect();
        sorted_chars.sort_unstable();
        let sorted_word: String = sorted_chars.into_iter().collect();
        anagram_map
            .entry(sorted_word)
            .or_default()
            .push(cleaned_word);
    }

    anagram_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_anagrams() {
        let filepath = "./test_words.txt";
        let anagrams = get_anagrams(filepath);

        let expected_anagrams = vec![
            (
                "ceiprstu",
                vec!["crepitus", "cuprites", "pictures", "piecrust"],
            ),
            (
                "aepst",
                vec![
                    "paste", "pates", "peats", "septa", "spate", "tapes", "tepas",
                ],
            ),
            ("ciilnoptu", vec!["punctilio", "unpolitic"]),
            ("denrssu", vec!["sunders", "undress"]),
        ];

        for (key, words) in expected_anagrams {
            let sorted_words = words.clone();
            // sorted_words.sort();
            let result_words = anagrams
                .get(key)
                .expect("oopsie doopsie there is no key in the anagram for that word")
                .clone();
            // result_words.sort();
            assert_eq!(sorted_words, result_words);
        }
    }
}
