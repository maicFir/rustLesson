// 申明一个new_test别名替代deeply::nested::test
use deeply::nested::test as new_test;

fn function_name() {
    println!("called `function()`");
}
fn function_name2() -> () {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn test() {
            println!("hello test")
        }
    }
}

fn main() {
    println!("Hello, world!");
    deeply::nested::test();
    new_test();
    function_name();
    function_name2();
}
