fn main() {
    // 定义函数
    let diary = |i| i;

    let diary2 = |i: i32| i + 1;

    // 定义一个函数
    fn diary3(i: i32) -> i32 {
        i + 1
    }
    let num: i32 = 10;
    println!("{}", diary("I love Rust"));
    println!("{}", diary2(num));
    println!("{}", diary3(num));
}
