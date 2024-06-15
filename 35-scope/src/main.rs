fn create_box() {
    let _age = Box::new(18i32);
    println!("{}", _age);
}
// 生命周期 'a生命周期

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    println!("Hello, world!");
    create_box();
    // _box3这一块作用域中
    {
        let _box3 = Box::new(3i32);
        println!("{}", _box3);
    }

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}
