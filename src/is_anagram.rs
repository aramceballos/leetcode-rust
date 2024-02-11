use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_chars_count: HashMap<char, u32> = HashMap::new();
    for c in s.chars().into_iter() {
        let prev = s_chars_count.entry(c).or_insert(0);
        *prev += 1;
    }
    let mut t_chars_count: HashMap<char, u32> = HashMap::new();
    for c in t.chars().into_iter() {
        let prev = t_chars_count.entry(c).or_insert(0);
        *prev += 1;
    }

    if s_chars_count.len() == t_chars_count.len() {
        for c in s_chars_count.keys().into_iter() {
            if s_chars_count.get(c) != t_chars_count.get(c) {
                return false;
            }
        }
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_anagram() {
        assert_eq!(
            is_anagram(String::from("listen"), String::from("silent")),
            true
        );
    }

    #[test]
    fn check_non_anagram() {
        assert_eq!(
            is_anagram(String::from("hello"), String::from("world")),
            false
        );
    }
}
