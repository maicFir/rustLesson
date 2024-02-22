fn main() {
    // for循环a..=b 表示[a,b]范围
    for item in 1..=10 {
        println!("item1: {}", item);
    }
    // a..b 表示[a,b)范围
    for item in (1..11).rev() {
        println!("item,{}", item);
    }
    // vec！申明一个可迭代器的变量
    let names = vec!["a", "b", "c"];
    for item in names.iter() {
        match *item {
            "a" => println!("There is a rustacean among us!, {}", item),
            _ => println!("Hello {}", item),
        }
    }
    let names2 = vec!["e", "f", "g"];
    // 在循坏into_iter()中移除,一旦在循环中被使用，就会被移除
    for name in names2.into_iter() {
        match name {
            "g" => println!("maicFir,there is a value among us!"),
            _ => println!("hello, {}", name),
        }
    }
    // iter_mut修改集合中的每个元素
    let mut names3 = vec!["Bob", "Frank", "Ferris"];
    for name in names3.iter_mut() {
        *name = match name {
            &mut "Ferris" => "hello",
            _ => "a",
        }
    }
    println!("names3:{:?}", names3);
}
