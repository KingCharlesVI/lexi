use std::collections::HashSet;

// Embed the dictionary file at compile time
const DICTIONARY: &str = include_str!("dictionary.txt");

pub struct WordDictionary {
    words: HashSet<String>,
}

impl WordDictionary {
    pub fn new() -> Self {
        let mut dictionary = WordDictionary {
            words: HashSet::new(),
        };
        dictionary.load_from_embedded();
        dictionary
    }

    fn load_from_embedded(&mut self) {
        for line in DICTIONARY.lines() {
            let word = line.trim().to_lowercase();
            if !word.is_empty() {
                self.words.insert(word);
            }
        }
    }

    pub fn find_matching_words(&self, pattern: &str) -> Result<Vec<String>, String> {
        if pattern.is_empty() {
            return Ok(Vec::new());
        }

        let pattern = pattern.to_lowercase();
        let mut results = self
            .words
            .iter()
            .filter(|word| word.contains(&pattern))
            .cloned()
            .collect::<Vec<_>>();

        results.sort_by(|a, b| a.len().cmp(&b.len()));
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matching_words() {
        let mut dict = WordDictionary {
            words: HashSet::new(),
        };
        dict.words.insert("apple".to_string());
        dict.words.insert("application".to_string());
        dict.words.insert("banana".to_string());

        let matches = dict.find_matching_words("app").unwrap();
        assert_eq!(matches.len(), 2);
        assert!(matches.contains(&"apple".to_string()));
        assert!(matches.contains(&"application".to_string()));

        let no_matches = dict.find_matching_words("xyz").unwrap();
        assert_eq!(no_matches.len(), 0);
    }
}