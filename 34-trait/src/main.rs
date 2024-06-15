// trait共享一个方法
trait Greet {
    fn greet(&self);
}
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}
// struct Person
struct Person {
    name: String,
}
// impl实现greet具体功能，使用for扩展Person的功能
impl Greet for Person {
    fn greet(&self) {
        println!("Hello, my name is {}!", self.name);
    }
}
fn main() {
    println!("Hello, world!");
    let person = Person {
        name: String::from("Alice"),
    };
    person.greet();
    let _x = ToDrop;
    // drop(_x);
}
