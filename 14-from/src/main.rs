use std::convert::From;

#[derive(Debug)]
struct NumberValue {
    value: i32,
}
impl From<i32> for NumberValue {
    fn from(item: i32) -> Self {
        NumberValue { value: item }
    }
}

fn main() {
    let my_name = "maic";
    let my_name_2 = String::from(my_name);
    let num = NumberValue::from(30);
    let int = 5;
    let new_num: NumberValue = int.into();
    let new_num2: i32 = "10".parse().unwrap();
    println!("the number value is {:?}", num.value);
    println!("new_num is {}", new_num.value);
    println!("{},{}", my_name, my_name_2);
    println!("new_num2={}", new_num2)
}
