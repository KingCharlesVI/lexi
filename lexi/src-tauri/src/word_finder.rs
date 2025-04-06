use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct WordDictionary {
    words: HashSet<String>,
}

impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary {
            words: HashSet::new(),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let word = line?.trim().to_lowercase();
            if !word.is_empty() {
                self.words.insert(word);
            }
        }

        Ok(())
    }

    pub fn find_matching_words(&self, pattern: &str) -> Result<Vec<String>, String> {
        if pattern.is_empty() {
            return Ok(Vec::new());
        }

        // Convert pattern to lowercase for case-insensitive matching
        let pattern = pattern.to_lowercase();
        
        // Build regex pattern - match words containing the pattern
        let mut results = Vec::new();
        
        for word in &self.words {
            if word.contains(&pattern) {
                results.push(word.clone());
            }
        }
        
        // Sort results by length (shortest first)
        results.sort_by(|a, b| a.len().cmp(&b.len()));
        
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matching_words() {
        let mut dict = WordDictionary::new();
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