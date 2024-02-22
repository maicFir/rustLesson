fn main() {
    let number = 10;
    let new_number = if number < 0 {
        println!("to small");
        -10
    } else {
        println!("to big");
        100
    };
    println!("{}", new_number);
}
