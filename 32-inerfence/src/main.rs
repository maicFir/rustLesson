use std::fmt::Display;

#[derive(Debug)]
struct A;
#[derive(Debug)]
struct Single(A);
#[derive(Debug)]
struct SingleGen<T>(T);

#[derive(Debug)]
struct Years(i64);

fn foo<T: Display>(arg: T) {
    println!("{}", arg)
}

impl Years {
    pub fn get_year(&self) -> i64 {
        self.0
    }
}

fn main() {
    let age = Years(27);
    // `Single` 是具体类型，并且显式地使用类型 `A`。
    let _s = Single(A);

    // 创建一个 `SingleGen<char>` 类型的变量 `_char`，并令其值为 `SingleGen('a')`
    // 这里的 `SingleGen` 的类型参数是显式指定的。
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` 的类型参数也可以隐式地指定。
    let _t = SingleGen(A); // 使用在上面定义的 `A`。
    let _i32 = SingleGen(6); // 使用 `i32` 类型。
    let _char = SingleGen('a'); // 使用 `char`。
    print!("{:?},{:?},{:?},{:?}", _s, _t, _i32, _char);
    print!("age:{:?}", age.get_year());
    foo("hello word")
}
