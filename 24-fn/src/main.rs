// 无返回值
fn fizebuzee_to(n: u32) -> () {
    println!("n={}", n);
    for item in 1..=n {
        println!("{}", item);
    }
}
// 表达式，没有分号，直接返回该值
fn test1() -> u32 {
    10
}
// return 返回
fn test2() -> u32 {
    return 100;
}
struct Point {
    x: f64,
    y: u32,
}
impl Point {
    fn origin() -> Point {
        Point { x: 1.2, y: 1 }
    }
    fn new(x: f64, y: u32) -> Point {
        Point { x, y }
    }
    fn get(&self) -> () {
        println!("x:{}, y:{}", self.x, self.y)
    }
}

// struct Pair(i32, i32);
// impl Pair {
//     fn destory(&self) {
//         let Pair(first, second) = self;
//         println!("destorying pair,{},{}", first, second);
//     }
// }
fn main() {
    fizebuzee_to(10);
    println!("test:{}", test1());
    println!("test2:{}", test2());
    let point_origin = Point::origin();
    let point_new = Point::new(1.2, 8);
    println!("{},{}", point_origin.x, point_origin.y);
    println!("{},{}", point_new.x, point_new.y);
}
