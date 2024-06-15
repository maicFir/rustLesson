fn private_function() {
    println!("call my_mod::private_function")
}
// 共有函数
pub fn public_function() {
    private_function();
    println!("call my_mod::public_function")
}
pub struct OpenBox<T> {
    pub contents: T,
}
#[allow(dead_code)]
pub struct ClosedBox<T> {
    pub contents: T, // 声明公有
}

impl<T> ClosedBox<T> {
    pub fn new(contents: T) -> ClosedBox<T> {
        ClosedBox { contents: contents }
    }
}
