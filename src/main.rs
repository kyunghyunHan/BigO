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
//O(n^2)
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

// #[allow(dead_code)]
// fn might_be_simple_caculate(array: Vec<i32>) -> i32 {
//     let mut total = 0;

//     for number in array.iter() {
//         let additional = if array.iter().position(|r| r == number).unwrap()

//         total = total + number + additional;
//     }
//     total
// }
#[allow(dead_code)]
fn realy_simple_caculate(array: Vec<i32>) -> i32 {
    let mut total = 0;

    for (index, number) in array.iter().enumerate() {
        let additional = if index > 5 { 5 } else { 1 };

        total = total + number + additional;
    }
    total
}
//n+n제곱에서 a를 2차원저긍로 0n 제곱
//o(n!)
#[allow(dead_code)]
fn not_simple_calculate(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;
    for number in array.iter() {
        println!("{}", number);
    }

    for i in 0..array.len() {
        for j in 0..array.len() {
            array[i] = array[i] + array[j];
        }
    }
    array
}
//O(1)
//O(log n)
//O(n)
//O(n^2)
//O(n^3)
//O(n!)
