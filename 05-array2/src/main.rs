use std::mem;

fn main() {
    // 申明数组的长度
    const DATA_NUMBER: [i32; 5] = [1, 2, 3, 4, 5];
    println!("data is length {}", DATA_NUMBER.len());
    // data在内存中
    println!("data is {:?}", mem::size_of_val(&DATA_NUMBER));
}
