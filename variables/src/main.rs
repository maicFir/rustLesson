fn main() {
    // let name = "Maic";
    // println!("hello,{}", name);
    // name = '2';
    let mut name = "Tom";
    println!("Hello, {}!", name);
    name = "Jake";

    println!("Hello, {}!", name);

    const NAME_RESULT: u32 = 60 * 60;

    let x: i32 = 5;

    let x = x + 1;
    {
        let x = x + 3;
        println!("hello x1={}", x);
    }

    println!("hello x2={}", x);

    println!("Hello, {}!", NAME_RESULT);

    let mut y: u32 = 5;
    println!("hello2, {}", y);
    y = 10;
    println!("hello2, {}", y);

    let str_name = "hello";
    let str_name = str_name.len();
    println!("hello length, {}", str_name);

    // 数据类型字符串类型
    let guess_number: &str = "12";
    let guess_number: u32 = guess_number.parse().expect("not a number");
    println!("guess number, {}", guess_number);
    // 浮点型
    let float_number = 2.5;
    let float_number2: f32 = 2.5;

    println!("float number, {}", float_number);
    println!("float number, {}", float_number2);

    // 布尔类型
    let is_true: bool = true;
    println!("is_true, {}", is_true);

    // 复合类型，将多个值组合成一个类型
    // rust中有两种复合类型，一种是元组，另一种是数组
    let tuple_number: (i32, f64, u8) = (800, 1.2, 5);
    println!("tuple_number-0, {}", tuple_number.0);
    println!("tuple_number-1, {}", tuple_number.1);
    println!("tuple_number-2, {}", tuple_number.2);

    //解构元数组
    let (x, y, z) = tuple_number;

    println!("tuple_number-0, {}", x);
    println!("tuple_number-1, {}", y);
    println!("tuple_number-2, {}", z);

    let name_array = [1, 2, 3, 4, 5];
    println!("name_array-0, {}", name_array[0]);
}
