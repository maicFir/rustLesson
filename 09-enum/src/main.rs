#![allow(dead_code)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char), // 元组结构体
    Paste(String),
    Click { x: i64, y: i64 },
}

enum VeryVerBoseEnum {
    Add,
    Subtract,
}

type OtherVeryVerBoseEnum = VeryVerBoseEnum;

// 将枚举WebEvent当成inspect的形参
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!("keypress:{}", c),
        WebEvent::Paste(s) => println!("Paste,{}", s),
        WebEvent::Click { x, y } => {
            println!("x={},y={}", x, y)
        }
    }
}
impl VeryVerBoseEnum {
    fn run(&self, x: i32, y: i32) {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        };
    }
}
fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my name".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    let x = OtherVeryVerBoseEnum::Add;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    println!("Hello, world!");
}
