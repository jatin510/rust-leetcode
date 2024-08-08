use std::collections::HashMap;

pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut results: Vec<String> = vec![];

    let mut hashmap: HashMap<String, i128> = HashMap::new();

    s1.split(" ").into_iter().for_each(|word| {
        let word = word.to_string();

        if hashmap.contains_key(word.as_str()) {
            hashmap.insert(word.to_string(), hashmap.get(word.as_str()).unwrap() + 1);
        } else {
            hashmap.insert(word.to_string(), 1);
        }
    });

    s2.split(" ").into_iter().for_each(|word| {
        let word = word.to_string();

        if hashmap.contains_key(&word) {
            hashmap.insert(word.to_string(), hashmap.get(&word).unwrap() + 1);
        } else {
            hashmap.insert(word.to_string(), 1);
        }
    });


    hashmap.iter().for_each(|(key, value)| {
        if *value == 1 {
            results.push(key.to_string());
        }
    });

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s1 = "this apple is sweet".to_string();
        let s2 = "this apple is sour".to_string();
        assert_eq!(uncommon_from_sentences(s1, s2), vec!["sweet".to_string(), "sour".to_string()]);
    }

    #[test]
    fn test_2() {
        let s1 = "apple apple".to_string();
        let s2 = "banana".to_string();
        assert_eq!(uncommon_from_sentences(s1, s2), vec!["banana".to_string()]);
    }
}