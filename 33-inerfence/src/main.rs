/**
 * 关联类型
 */
struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}
impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}
fn main() {
    let c = Container(5, 10);
    println!("{}", c.contains(&5, &10));
    println!("first:{}", c.first());
    println!("last:{}", c.last());
}
