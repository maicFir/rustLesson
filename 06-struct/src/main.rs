#[derive(Debug)]

struct Person {
    name: String,
    age: i8,
}
// 单元结构体
// struct Unit;

// 元组结构体
struct Pair(i32, f32);
// struct 带有两个字段的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
// struct Rectangle {
//     top_left: Point,
//     bottom_right: Point,
// }
fn main() {
    let name = String::from("Maic");
    let age = 18;
    let user_info = Person { name, age };
    println!(
        "user_info.name:{},user_info.age:{}",
        user_info.name, user_info.age
    );
    let point = Point { x: 1.2, y: 3.4 };
    let point2 = Point { x: 5.6, ..point };
    let pair = Pair(1, 2.0);
    // 解构pair数据
    let Pair(x1, y1) = pair;
    // 解构Point
    let Point { x, y } = point;
    println!("point:{:?}", point);
    println!("x={},y={}", point.x, point.y);
    println!("x={},y={}", point2.x, point2.y);
    println!("x1={},x2={}", pair.0, pair.1);
    println!("x1={},y1={}", x1, y1);
    println!("x3,{},y3,{}", x, y);
    println!("Hello, world!");
}
