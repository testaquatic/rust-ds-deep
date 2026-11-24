//! Robin Hood hasing 개념 시연 (단순화한 버전)
const CAPACITY: usize = 8;

#[derive(Clone, Debug)]
struct Slot {
    key: Option<String>,
    value: i32,
    /// 이 슬롯까지 탐사한 거리
    probe_list: usize,
}

impl Slot {
    fn empty() -> Self {
        Slot {
            key: None,
            value: 0,
            probe_list: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.key.is_none()
    }
}

fn hash(key: &str) -> usize {
    // FNV offset basis
    let mut h = 0xcbf29ce484222325_u64;

    for b in key.bytes() {
        h ^= b as u64;
        // FNV prime
        h = h.wrapping_mul(0x100000001b3);
    }

    (h as usize) % CAPACITY
}

fn rh_insert(table: &mut [Slot; CAPACITY], key: String, value: i32) {
    let mut idx = hash(&key);
    let mut current_key = key;
    let mut current_val = value;
    let mut dist = 0_usize;

    loop {
        let slot = &mut table[idx % CAPACITY];

        if slot.is_empty() {
            // 빈 슬롯이므로 그냥 삽입
            *slot = Slot {
                key: Some(current_key),
                value: current_val,
                probe_list: dist,
            };

            return;
        }

        // 이미 차 있다면
        if slot.key.as_deref() == Some(&current_key) {
            // 같은 키: 값 갱신
            slot.value = current_val;
            return;
        }

        // Robin Hood 조건: 현재 삽입 중인 항목이 더 멀리 왔으면 자리 교체
        if dist > slot.probe_list {
            let old_key = slot.key.take().unwrap();
            let old_val = slot.value;
            let old_dist = slot.probe_list;

            *slot = Slot {
                key: Some(current_key),
                value: current_val,
                probe_list: dist,
            };

            current_key = old_key;
            current_val = old_val;
            dist = old_dist;
        }

        idx += 1;
        dist += 1;
    }
}

fn main() {
    let mut table = std::array::from_fn::<_, CAPACITY, _>(|_| Slot::empty());

    let keys = ["apple", "banana", "cherry", "date", "elderberry"];
    for (i, key) in keys.iter().enumerate() {
        rh_insert(&mut table, key.to_string(), i as i32 * 10);
    }

    println!("=== Robin Hood 테이블 상태 ===");
    for (i, slot) in table.iter().enumerate() {
        if let Some(ref k) = slot.key {
            println!("slot[{}]: key={:12} dist={}", i, k, slot.probe_list);
        } else {
            println!("slot[{}]: (empty)", i);
        }
    }
}
