fn main() {
    let mut source = Some(0);
    loop {
        match source {
            Some(i) => {
                if i > 8 {
                    println!("more then 8");
                    source = None;
                } else {
                    println!("`i` is {:?}", i);
                    source = Some(i + 1);
                }
            }
            _ => {
                break; // 解构失败时退出循环
            }
        }
    }

    let mut source2 = Some(0);
    // 使用while替换loop,申请可变source2
    while let Some(i) = source2 {
        if (i > 8) {
            println!("Greater than 8,quit");
            source2 = None;
        } else {
            println!("`i` is {}", i);
            source2 = Some(i + 1);
        }
    }
}
