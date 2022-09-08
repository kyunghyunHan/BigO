use std::array;

fn main() {
    println!("Hello, world!");
}
//알고리즘의 복잡성

//주어진 모든 숫자의 합을 계산
//변수로 선언된 배열
//입력데이터에 대한 선형 의존성
//표기법
//O(n)  O(log(n))
#[allow(dead_code)]
fn sum_numbers_in_array(array: Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in array.iter() {
        sum += i;
    }
    sum
}

//o from uinty
//상수의 복잡도
// fn simple_calculate() -> i32 {
//     const A: i32 = 1 + 2;
//     const B: i32 = 3 + 4;

//     println!("calculating...");
//     B - A
// }
#[allow(dead_code)]
fn simple_calculate(number: i32) -> i32 {
    const A: i32 = 1 + 2;
    const B: i32 = 3 + 4;

    println!("calculating...");
    B - A + number
}
//입력이 배열이고 복잡성은 항상 n에서 선형
#[allow(dead_code)]
fn simple_calculate_array_sum(array: Vec<i32>) -> i32 {
    const A: i32 = 1 + 2;
    const B: i32 = 3 + 4;

    let mut additional_nuber = 0;

    for i in array.iter() {
        additional_nuber += i;
    }

    B - A + additional_nuber
}

#[allow(dead_code)]
fn not_so_simple_calculate(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;

    for i in 0..array.len() {
        for j in 0..array.len() {
            array[i] = array[i] + array[j];
        }
    }
    array
}
