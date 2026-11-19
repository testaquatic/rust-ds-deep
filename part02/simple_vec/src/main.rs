use simple_vec::SimpleVec;

fn main() {
    let mut v = SimpleVec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    for i in 0..3 {
        if let Some(val) = v.get(i) {
            println!("[{}] = {}", i, val);
        }
    }
}
