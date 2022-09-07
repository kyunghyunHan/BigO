fn main() {
    println!("Hello, world!");
}
//알고리즘의 복잡성

//주어진 모든 숫자의 합을 계산
//변수로 선언된 배열
//입력데이터에 대한 선형 의존성
#[allow(dead_code)]
// fn sum_numbers_in_array(array: Vec<i32>) -> i32 {
//     let mut sum = 0;

//     for i in array.iter() {
//         sum += i;
//     }
//     sum
// }

fn simple_calculate() -> i32 {
    const A: i32 = 1 + 2;
    const B: i32 = 3 + 4;

    println!("calculating...");
    B - A
}
