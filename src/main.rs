fn main() {
    println!("Hello world");
    //배열 연습
    let mut dp: [i8; 10] = [42; 10]; //10만큼 크기의 배열을 생서앟고 42를 10번 할당
    let mut arr: Vec<i32> = Vec::new();

    // 배열에 요소 추가
    arr.push(1);
    arr.push(2);
    arr.push(3);
    arr.push(4);
    arr.push(5);

    println!("배열: {:?}", arr);

    // 배열에서 요소 제거
    arr.pop(); // 마지막 요소 제거
    println!("마지막 요소 제거 후 배열: {:?}", arr);

    // 요소 변경
    if let Some(e) = arr.get_mut(2) {
        *e = 10;
    }
    println!("변경 후 배열: {:?}", arr);
    print!("{}", sum(1, 3));
}

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    return a + b
}