pub mod nested {
    pub fn get_hello() {
        println!("hello world from nested module")
    }
    pub trait Hello {
        fn say_hello(&self);
    }
    pub struct HelloMaic;
    impl Hello for HelloMaic {
        fn say_hello(&self) {
            println!("Hello Maic");
        }
    }
}
