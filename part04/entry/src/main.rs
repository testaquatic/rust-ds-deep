use std::collections::{HashMap, hash_map::Entry};

fn main() {
    let mut inventory = HashMap::new();

    match inventory.entry("apple") {
        Entry::Occupied(mut e) => {
            println!("apple 존재: {}", e.get());
            *e.get_mut() += 10;
        }
        Entry::Vacant(e) => {
            println!("apple 없음 -> 초기화");
            e.insert(100);
        }
    }

    match inventory.entry("apple") {
        Entry::Occupied(mut e) => {
            println!("apple 이번엔 존재: {}", e.get());
            *e.get_mut() += 50
        }
        Entry::Vacant(_) => (),
    }

    println!("apple 재고: {}", inventory["apple"]);

    inventory
        .entry("banana")
        .and_modify(|v| *v *= 2)
        .or_insert(50);

    inventory
        .entry("apple")
        .and_modify(|v| *v *= 2)
        .or_insert(100);

    println!("apple: {}", inventory["apple"]);
    println!("banana: {}", inventory["banana"]);
}
