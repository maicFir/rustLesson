fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer; // 退出外层循环
        }
        println!("This point will never be reached"); // 退出outer循环了，就不能打印了
    }
    println!("Exited the outer loop");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
    println!("{}", counter);

    let mut num = 3;
    while num < 101 {
        if num % 15 == 0 {
            println!("fizzbuzz, {}", num);
        } else if num % 3 == 0 {
            println!("fizz, {}", num);
        } else if num % 5 == 0 {
            println!("buzz,{}", num);
        } else {
            println!("{}", num)
        }
        num += 1;
    }
    println!("num:{}", num);
}
