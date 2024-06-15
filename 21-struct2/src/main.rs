struct Stu {
    name: String,
    age: i32,
}
struct Info(i32, f32);
fn main() {
    let user = Stu {
        name: String::from("Maic"),
        age: 18,
    };

    let user2 = Stu {
        name: String::from("Tom"),
        ..user
    };
    // 解构
    let Stu { ref age, ref name } = user;

    let stu_info = Info(18, 20.1);

    let Info(stu_number, stu_score) = stu_info;
    println!("{},{}", user.name, user.age);
    println!("name:{},age:{}", name, age);
    println!("user2:{}, {}", user2.name, user2.age);
    println!("stuInfo: {}, {}", stu_info.0, stu_info.1);
    println!("stu_number:{}, stu_score:{}", stu_number, stu_score);
}
