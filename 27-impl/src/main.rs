struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
    // 判断是否是正方形
    fn get_is_square(&self) -> bool {
        self.width == self.height
    }
    // 关联函数
    fn get_area_associated_func(width: u32, height: u32) -> Rect {
        Rect { width, height }
    }
}
fn main() {
    println!("Hello, world!");
    let rect = Rect {
        width: 30,
        height: 50,
    };
    let rect2: Rect = Rect { width: 5, ..rect };
    let srect = Rect::get_area_associated_func(30, 50);
    println!("Area: {}", rect.get_area());
    println!("Is square: {}", rect.get_is_square());
    println!("width: {},height:{}", rect2.width, rect2.height);
    println!("width: {:?},height:{:?}", srect.width, srect.height);
}
