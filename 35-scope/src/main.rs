fn create_box() {
    let _age = Box::new(18i32);
    println!("{}", _age);
}
fn main() {
    println!("Hello, world!");
    create_box();
    // _box3这一块作用域中
    {
        let _box3 = Box::new(3i32);
        println!("{}", _box3);
    }
}
