use linked_list_box::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    println!("길이: {}", list.len());

    for val in list.iter() {
        println!("{}", val);
    }
    println!();

    println!("peak: {:?}", list.peek_front());
    println!("pop: {:?}", list.pop_front());
    println!("pop: {:?}", list.pop_front());

    list.push_front(10);
    list.push_front(20);
    for val in list {
        println!("{}", val);
    }
}
