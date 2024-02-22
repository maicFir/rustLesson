use List::*;

enum List {
    Maic(u32, Box<List>),
    Nil,
}
impl List {
    fn new_list() -> List {
        Nil
    }
    fn prepend(self, val: u32) -> List {
        Maic(val, Box::new(self))
    }
    fn len(&self) -> u32 {
        match *self {
            Maic(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Maic(head, ref tail) => {
                format!("{},{}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}
fn main() {
    println!("Hello, world!");
    let mut list = List::new_list();
    list = list.prepend(1);
    list = list.prepend(2);
    println!("linked list has length:{}", list.len());
    println!("{}", list.stringify());
}
