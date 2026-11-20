use mini_vec::MiniVec;

fn main() {
    let mut v = MiniVec::new();
    println!("초기: len={}, cap={}", v.len(), v.capacity());

    for i in 0..10 {
        v.push(i);
        println!("push({}) -> len={}, cap={}", i, v.len(), v.capacity());
    }

    println!("get(3) = {:?}", v.get(3));
    println!("get(99) = {:?}", v.get(99));

    println!("pop = {:?}", v.pop());
    println!("pop = {:?}", v.pop());
    println!("len={}, cap={}", v.len(), v.capacity());
}
