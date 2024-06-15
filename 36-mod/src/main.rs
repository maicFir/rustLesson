use my::nested::Hello;
// 会找到my.rs或者my/mod.rs文件
mod my;

mod my_mod;

// mod my_mod {
//     fn private_function() {
//         println!("call my_mod::private_function")
//     }
//     // 共有函数
//     pub fn public_function() {
//         private_function();
//         println!("call my_mod::public_function")
//     }
//     pub struct OpenBox<T> {
//         pub contents: T,
//     }
//     #[allow(dead_code)]
//     pub struct ClosedBox<T> {
//         pub contents: T, // 声明公有
//     }

//     impl<T> ClosedBox<T> {
//         pub fn new(contents: T) -> ClosedBox<T> {
//             ClosedBox { contents: contents }
//         }
//     }
// }
fn main() {
    // 会报错
    // my_mod::private_function();
    my_mod::public_function();

    let open_box = my_mod::OpenBox {
        contents: "hello Maic",
    };
    let close_box = my_mod::ClosedBox::new("Best for you");
    println!("{}", open_box.contents);
    println!("{}", close_box.contents);

    my::nested::get_hello();

    let maic = my::nested::HelloMaic;
    maic.say_hello();
}
