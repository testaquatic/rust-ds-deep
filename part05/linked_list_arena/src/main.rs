use linked_list_arena::ArenaList;

fn main() {
    let mut list = ArenaList::new();

    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    println!("리스트: {:?}", list.iter().collect::<Vec<_>>());

    list.remove(&2);
    println!("2 제거 후: {:?}", list.iter().collect::<Vec<_>>());

    while let Some(val) = list.pop_front() {
        println!("pop: {}", val);
    }

    println!("비었나요? {}", list.is_empty());
}
