fn main() {
    let int:i32 = 1;
    let long:i64 = 123123123123123;
    let float:f64 = 10.123;
    let text_data: char = '러';

    println!("i32 = int, {}", int);
    println!("i64 = long, {}", long);
    println!("f64 = double, {}", float);
    println!("char = char, {}", text_data);

    println!("i32:max = {}, min = {}", i32::MAX, i32::MIN);
    println!("u32:max = {}, min = {}", u32::MAX, u32::MIN);
    println!("f64:max = {}, min = {}", f64::MAX, f64::MIN);

    // 오버플로우 발생 시 컴파일 에러
    // let overflow_data:i8 = i8::MAX + 1;
    // println!("{}", overflow_data);
}
