/**
 * 切片
 */
fn main() {
    let arr = [1, 2, 3, 4, 5, 6];
    let slice = &arr[1..4];
    let slice2 = &arr[..4];
    let slice3 = &arr[1..];
    println!("slice:{:?}", slice); // [2,3,4]
    println!("slice2:{:?}", slice2); // [1,2,3,4]
    println!("slice3:{:?}", slice3); // [2,3,4,5,6]
}
