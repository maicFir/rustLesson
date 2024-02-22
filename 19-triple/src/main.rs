fn main() {
    let triple = (0, 1, 1.3);
    println!("triple,{:?}", triple);
    match triple {
        (0, ..) => println!("first is zero"),
        (1, ..) => println!("first is one"),
        (.., z) if z < 0.0 => println!("last is negative"),
        _ => println!("otherwise"),
    }
}
