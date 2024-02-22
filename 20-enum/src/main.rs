// 消除警告，使用枚举类型取值
#[allow(dead_code)]
// 加了这个不会报错,第15行不会报错
#[derive(Debug)]
enum Color {
    Red,
    Blue,
}

fn main() {
    // &表示取引用
    let reference = &4;
    let color = Color::Red;
    match color {
        Color::Red => println!("color is red,{:?}", color),
        Color::Blue => println!("color is blue"),
    }
    match reference {
        // &val指向的是reference
        &val => println!("value is {}", val),
    }
    // 等价于上面，使用*引用，这样val不用加&
    match *reference {
        val => println!("value2 is {}", val),
    }
    // 没有引用
    let no_reference = 3;
    match no_reference {
        val => println!("value3 is {}", val),
    }
    match no_reference {
        ref val => println!("value4 is {}", val),
    }
    let mut value = 10;

    match value {
        ref mut val => {
            // 先用*解引用，然后+10
            *val += 10;
            println!("value5 is {}", val);
        }
    }
}
