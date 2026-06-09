use linked_list_rc_refcell::DoublyLinkedList;

fn main() {
    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_front(0);

    println!("앞 -> 뒤: {:?}", list.to_vec_forward());
    println!("뒤 -> 앞: {:?}", list.to_vec_backward());

    println!("pop_back: {:?}", list.pop_back());
    println!("pop_front: {:?}", list.pop_front());

    println!("길이 : {}", list.len());
    println!("앞 -> 뒤: {:?}", list.to_vec_forward());
}
