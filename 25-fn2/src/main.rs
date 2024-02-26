fn main() {
    let rect = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("area1:{}", set_area1(3.0, 4.0));
    println!("area2:{}", set_area2((3.0, 4.0)));
    // {:#?}会输出有格式的
    println!("area3:{:#?}", rect);
    println!("area3:{}", set_area3(&rect));
}

fn set_area1(w: f64, h: f64) -> f64 {
    // 计算矩形面积
    w * h
}
// 使用元组
fn set_area2(react: (f64, f64)) -> f64 {
    react.0 * react.1
}
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}
// 使用struct

fn set_area3(rect: &Rectangle) -> f64 {
    rect.width * rect.height
}
