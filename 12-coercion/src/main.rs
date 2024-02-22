fn main() {
    let decimal = 65.4321_f32;

    println!("{}", decimal);
    let integer = decimal as u8;
    let inter_character = integer as char;

    // 65 - A
    println!("Casting:{}->{}", integer, inter_character);

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    // x占用字节
    println!("size of x={}", std::mem::size_of_val(&x));
    // y占用字节
    println!("size of y={}", std::mem::size_of_val(&y));
    // z占用字节
    println!("size of z={}", std::mem::size_of_val(&z));
}
