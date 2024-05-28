fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn call_me<F: Fn()>(f: F) {
    f();
}

fn create_fnmut() -> impl FnMut() {
    let mut counter = 0;

    // 返回一个闭包，它会修改它所捕获的变量。
    // 因为 `FnMut` 生命周期参数是必须的，所以我们可以省略它。
    move || {
        counter += 1;
        println!("Counter: {}", counter);
    }
}
fn main() {
    let x = 8;
    // 捕获 `x` 到匿名类型中，并为它实现 `Fn`。
    // 将闭包存储到 `print` 中。
    let print = || println!("x is {}", x);

    let mut fn_mut = create_fnmut();
    fn function() {
        println!("This is a function!");
    }
    apply(print);
    call_me(function);
    fn_mut();
}
