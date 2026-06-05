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
        let mut h = 0_u64;
        for b in key.bytes() {
            h = h.wrapping_mul(31).wrapping_add(b as u64);
        }

        (h as usize) % self.capacity
    }

    fn insert(&mut self, key: String, value: i32) {
        let idx = self.hash(&key);

        for pair in &mut self.buckets[idx] {
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }

        self.buckets[idx].push((key, value));
    }

    fn get(&self, key: &str) -> Option<i32> {
        let idx = self.hash(key);

        for pair in &self.buckets[idx] {
            if pair.0 == key {
                return Some(pair.1);
            }
        }

        None
    }
}

fn main() {
    let mut map = ChainingMap::new(8);
    map.insert("alice".into(), 30);
    map.insert("bob".into(), 28);
    map.insert("carol".into(), 25);

    println!("alice: {:?}", map.get("alice"));
    println!("dave: {:?}", map.get("dave"));
}
