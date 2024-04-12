fn main() {
    let tuple = (1, 2, 3);
    let explicit_type_tuple: (i32, f64, char) = (123, 123.456, 'X');
    // 튜플에 직접 접근은 불가능
    // println!("{}", tuple);
    
    // 배열처럼 0부터 시작하는 인덱스를 가지고 접근할 수 있음.
    println!("tuple : {}, {}, {}", tuple.0, tuple.1, tuple.2);
    println!("explicit_type : {}, {}, {}", explicit_type_tuple.0, explicit_type_tuple.1, explicit_type_tuple.2);
}
