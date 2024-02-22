fn print_name(name: &str) {
    println!("{}", name);
    text();
    println!("{}", add_one(3)); // 调用add_one函数
}
fn text() {
    let x = {
        let y = 1;
        y + 1
    };
    println!("{}", x);
}
// 不加分号，add_one返回的值是5
fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    print_name("Maic");
}
