// 创建一个结构体
struct Student {
    name: String,
    age: u32,
    marks: u32,
}

fn get_age() -> u32 {
    10
}
fn main() {
    let lihua: Student = Student {
        name: String::from("Li Hua"),
        age: 20,
        marks: 90,
    };
    println!("{},{},{}", lihua.name, lihua.age, lihua.marks);
    // 解构结构体,如果后期有同时解构，则前面一个要使用ref
    let Student {
        name: ref iname,
        age,
        marks,
    } = lihua;
    println!("name:{},age:{},marks:{}", iname, age, marks);
    let Student { name: ref n, .. } = lihua;
    println!("name:{}", n);

    // 匹配元组
    let pair = (1, 2);
    match pair {
        (x, y) if x == y => println!("x==y"),
        (x, y) if x != y => println!("x!=y"),
        _ => println!("others"),
    }

    let cage: u32 = get_age();

    match cage {
        0 => println!("age is 0"),
        n @ 1..=10 => println!("age is {}", n),
        // 没有匹配上的会走这里
        n => println!("age is {}", n),
    }
}
