use std::collections::HashMap;
/*
 * Implementation of Trie adapted for HackerRank challenge: https://www.hackerrank.com/challenges/no-prefix-set/problem
*/
pub struct Trie {
    is_word: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            is_word: false,
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        if word.is_empty() {
            self.is_word = true;
        } else {
            let c = word.chars().next().unwrap();
            self.children
                .entry(c)
                .or_insert_with(Trie::new)
                .insert(&word[1..]);
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        if word.is_empty() {
            self.is_word
        } else {
            let c = word.chars().next().unwrap();
            self.children
                .get(&c)
                .map(|child| child.contains(&word[1..]))
                .unwrap_or(false)
        }
    }

    pub fn has_prefix(&self, word: &str) -> bool {
        if word.is_empty(){
            false
        }
        else if !word.is_empty() && self.is_word {
            true
        } else {
            let c = word.chars().next().unwrap();
            self.children
                .get(&c)
                .map(|child| child.has_prefix(&word[1..]))
                .unwrap_or(false)
        }
    }
}
