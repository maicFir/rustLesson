#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}
fn main() {
    use Status::{Poor, Rich};
    // 引入所有
    use Work::*;

    let status = Status::Poor;
    // 与下面等价
    // let status2 = Poor;

    let work_civilian = Civilian;
    // let work_soldier = Soldier;

    match status {
        Rich => println!("the rich have lots of money..."),
        Poor => println!("the poor have no money..."),
    }
    match work_civilian {
        Civilian => println!("the civilian"),
        Soldier => println!("Soldiers fight!"),
    }
    println!("Hello, world!");
}
