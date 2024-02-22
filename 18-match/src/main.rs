fn main() {
    let number = 2;
    println!("number is {}", number);
    match number {
        1 => println!("first one"),
        2 | 3 => println!("{}", number),
        5..=6 => println!("good luck"),
        _ => println!("other case!"),
    };

    let boolean = false;
    let binary = match boolean {
        false => "false",
        true => "true",
    };
    println!("{} -> {}", boolean, binary);
}
