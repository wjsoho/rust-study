fn main() {
    demo1();
    demo2();
}

// 元组声明和元组元素获取
fn demo1() {
    // 1.每个元组元素都可以单独声明具体的数据类型否则就是默认类型
    let tup: (i32, f32, u8) = (32, 3.2, 8);
    println!("demo1输出1：tup={:?}",tup);
    // 使用".下标"来解析元组对应位置值，从0下标开始
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("demo1输出2：x={},y={},z={}", x, y, z);
    // 使用模式匹配来解析元组
    let (x, y, z) = tup;
    println!("demo1输出3：x={},y={},z={}", x, y, z);
}

// 数组声明和元素获取
fn demo2() {
    // 1.省略数组类型写法，默认为i32,长度为后面赋值的实际个数数量
    let a = [0, 1, 2, 3];
    println!("demo2输出1：a={:?}", a);
    // 2.指明数组元组类型为i8,长度为3
    let b: [i8; 3] = [0, 1, 2];
    println!("demo2输出2：b={:?}", b);
    // 3.声明数组类型为可变，因为默认是不可变
    let mut c = [1, 2, 3];
    println!("demo2输出3：c={:?}", c);
    // 4.通过0下标值由原来1修改为11
    c[0] = 11;
    println!("demo2输出4：c={:?}", c);
    // 5.指定元素值全为"d"及长度为5,";"分隔
    let d1 = ["d"; 5];
    println!("demo2输出5：d1={:?}",d1);
    // 等价于下面写法
    let d2 = ["d", "d", "d", "d", "d"];
    println!("demo2输出5：d2={:?}",d2);
    // 注意数组下标不能超过数组实际大小否则报panic错误，如下面注释
    // 运行时报报错：index out of bounds: the length is 5 but the index is 10
    // let element = d[10];
}
