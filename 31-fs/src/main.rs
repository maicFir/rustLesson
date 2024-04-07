mod my;

fn main() {
    println!("Hello, world!");
    let hello = my::indirect_access();
    println!("{}", hello);
}
