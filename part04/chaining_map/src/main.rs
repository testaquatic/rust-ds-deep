use std::vec;

struct ChainingMap {
    buckets: Vec<Vec<(String, i32)>>,
    capacity: usize,
}

impl ChainingMap {
    fn new(capacity: usize) -> Self {
        Self {
            buckets: vec![vec![]; capacity],
            capacity,
        }
    }

    fn hash(&self, key: &str) -> usize {
        let h = key
            .bytes()
            .fold(0_u64, |h, b| h.wrapping_mul(31).wrapping_add(b as u64));

        (h as usize) % self.capacity
    }

    fn insert(&mut self, key: String, value: i32) {
        let idx = self.hash(&key);
        // 기존 키가 있으면 갱신
        if let Some(pair) = self.buckets[idx].iter_mut().find(|pair| pair.0 == key) {
            pair.1 = value;
        }

        self.buckets[idx].push((key, value));
    }

    fn get(&self, key: &str) -> Option<i32> {
        let idx = self.hash(key);
        self.buckets
            .get(idx)?
            .iter()
            .find(|pair| pair.0 == key)
            .map(|pair| pair.1)
    }
}

fn main() {
    let mut map = ChainingMap::new(8);
    map.insert("alice".to_string(), 30);
    map.insert("bob".to_string(), 28);
    map.insert("carol".to_string(), 25);

    // Some(30)
    println!("alice: {:?}", map.get("alice"));
    // None
    println!("dave: {:?}", map.get("dave"));
}
