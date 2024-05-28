mod my_mod {
    fn private_function() {
        println!("1:调用了 my_mod 中的 private_function");
    }
    pub fn public_function() {
        private_function();
        println!("2:调用了 my_mod 中的 public_function");
    }

    // 内部模块的子模块
    pub mod nested_mod {
        pub fn function() {
            println!("3:调用了 my_mod::nested_mod::function");
            private_function(); // 可以访问nested_mod模块中的私有方法
        }
        #[allow(dead_code)]
        fn private_function() {
            println!("4:called `my_mod::nested::private_function()`");
        }
    }
    pub fn indirect_access() {
        print!("5:called `my_mod::indirect_access()`, that\n> ");
        nested_mod::function();
    }
}

fn main() {
    println!("Hello, world!");
    my_mod::public_function(); // 调用公有函数
                               // my_mod::private_function(); // 报错，私有函数无法直接访问
    my_mod::indirect_access(); // 间接调用公有函数
}
