use std::collections::HashMap;

struct MagicDictionary {
    tree: HashMap<char, Box<MagicDictionary>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            tree: HashMap::default(),
        }
    }
    /** Build a dictionary through a list of words */
    fn build_dict(&self, dict: Vec<String>) {}
    /** Returns if there is any word in the trie that equals to the given word after modifying exactly one character */
    fn search(&self, word: String) -> bool {
        self.search_recrusive(&word, 0)
    }

    fn search_recrusive(&self, word: &str, err: u32) -> bool {
        if word.len() == 0 {
            if self.tree.len() == 0 && err == 1 {
                return true;
            }
        }

        false
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dict);
 * let ret_2: bool = obj.search(word);
 */

fn main() {
    println!("hello, world!");
}
