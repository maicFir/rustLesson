use std::io;
fn main() {
    // 申明一个输入的字符串

    let luck_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    loop {
        let mut input_number = String::new();
        println!("请输入一个数字");
        // 读取用户输入
        io::stdin().read_line(&mut input_number).expect("读取失败");
        // 将输入的字符串转换为整数
        let number: usize = input_number.trim().parse().expect("input number is error");
        println!("你的幸运数字是：{}", luck_array[number]);
    }
}
