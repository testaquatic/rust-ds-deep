struct Padded {
    a: u8,
    b: u64,
}

struct Compact {
    b: u64,
    a: u8,
}

fn main() {
    println!("Padded size:  {}", std::mem::size_of::<Padded>());
    println!("Compact size: {}", std::mem::size_of::<Compact>());
}
