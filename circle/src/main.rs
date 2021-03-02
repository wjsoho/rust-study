fn main() {
    demo1();
    demo2();
    demo3();
}

// while循环
fn demo1() {
    let arr = [1, 2, 3, 4, 5, 6];
    let mut i = 0;
    while i < 6 {
        println!("demo1输出，value i is {}", arr[i]);
        i = i + 1;
    }
}

// for循环
fn demo2() {
    let arr = [1, 2, 3, 4, 5, 6];
    for element in arr.iter() {
        println!("demo2输出，element:{}", element)
    }
}

// loop循环
fn demo3() {}