fn test_maic() {
    println!("This is a test maic");
}
mod cool {
    mod sub_cool {
        // 私有访问
        pub fn test_private2() {
            println!("This is a private test2");
        }
    }
    // pub申明一个公有的test方法
    pub fn test() {
        println!("This is a test");
        // 私有函数在同一模块内调用
        self::test_private();
        // 访问内部模块sub_cool，并调用公有函数
        self::sub_cool::test_private2();
        // cool模块外，使用super调用函数
        super::test_maic();
    }
    // 私有函数，在外部不能直接访问
    fn test_private() {
        println!("This is a private test");
    }
}

fn main() {
    println!("Hello, world!");
    cool::test();
    // cool::test_private();
}
