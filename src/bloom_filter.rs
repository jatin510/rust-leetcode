use std::hash::{DefaultHasher, Hash, Hasher};

pub struct BloomFilter {
    pub bit_array: Vec<bool>,
    pub size: usize,
    pub hash_count: usize,
}

impl BloomFilter {
    pub fn new(size: usize, hash_count: usize) -> Self {
        Self {
            bit_array: vec![false; size],
            size,
            hash_count,
        }
    }

    pub fn add(&mut self, key: &str) {
        for i in 0..self.hash_count {
            let index = self.hash(key, i);
            self.bit_array[index] = true;
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        for i in 0..self.hash_count {
            let index = self.hash(key, i);
            if self.bit_array[index] == false {
                return false;
            }
        }

        return true;
    }

    fn hash(&self, value: &str, i: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        let salt = i.to_string();
        value.hash(&mut hasher);
        salt.hash(&mut hasher);
        hasher.finish() as usize % self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bloom_filter() {
        let mut bloom_filter = BloomFilter::new(10, 3);
        assert_eq!(bloom_filter.size, 10);
        assert_eq!(bloom_filter.hash_count, 3);
        assert_eq!(bloom_filter.bit_array, vec![false; 10]);
    }

    #[test]
    fn test_bool_filter() {
        let mut bloom_filter = BloomFilter::new(10, 3);
        bloom_filter.add("hello");
        bloom_filter.add("world");

        assert_eq!(bloom_filter.contains("hello"), true);
        assert_eq!(bloom_filter.contains("world"), true);
        assert_eq!(bloom_filter.contains("jatin"), false);
    }
}
