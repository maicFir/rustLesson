fn main() {
    let x = 1;
    let y = 2;
    if x > y {
        println!("x is greater than y");
    } else {
        println!("x is less than y");
    }
    let flag = true;
    let result = if flag { 5 } else { 6 };
    println!("result is {}", result);
}
