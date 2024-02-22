enum Name {
    A,
    B,
    C,
    Qux(u32),
}

fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("this is a really long string and {:?}", i);
        }
        _ => {
            println!("other case"); // 必须要有
        }
    };
    // 可以用下面替代上面的match
    if let Some(i) = optional {
        println!("matched{:?}", i)
    }
    let letter: Option<i32> = None;
    // 这里Some中的i不能与上面的i一致
    // 这里不能是if let xx == letter
    if let Some(_i) = letter {
        println!("true")
    } else {
        println!("false");
    }
    let _a = Name::A;
    let _b = Name::B;
    let _c = Name::C;
    let _d = Name::Qux(100);

    if let Name::A = _a {
        println!("this is right value");
    }
    // 枚举直接比较会报错
    // if Name::A == _a {
    //     println!("_a is true");
    // }
    if let Name::Qux(value) = _d {
        println!("c is {}", value);
    }
}
