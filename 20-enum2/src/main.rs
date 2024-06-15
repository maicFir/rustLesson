#[warn(dead_code)]
#[derive(Debug)]
enum LoadingStatus {
    Loading,
    Loaded,
    Error,
    Pasted(String),
    Point { x: i64, y: i64 },
}

fn log(status: LoadingStatus) {
    match status {
        LoadingStatus::Loading => println!("loading, {:?}", LoadingStatus::Loading),
        LoadingStatus::Loaded => println!("loaded"),
        LoadingStatus::Error => println!("error"),
        LoadingStatus::Pasted(c) => println!("pasted {}", c),
        LoadingStatus::Point { x, y } => println!("point {},{}", x, y),
    }
}
fn main() {
    println!("Hello, world!");
    let point = LoadingStatus::Point { x: 1, y: 2 };
    let loading = LoadingStatus::Loading;
    let pasted = LoadingStatus::Pasted(String::from("Maic"));
    log(point);
    log(loading);
    log(pasted);
}
