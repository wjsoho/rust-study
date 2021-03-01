fn main() {
    demo1(5);
    demo2();
    println!("demo3：{}",demo3())
}

// 1.函数定义
fn demo1(x: i32) {
    println!("demo1:The value of x is: {}", x);
}

// 2.函数体语句与表达式
fn demo2() {
    // 1.下面定义x就是一个语句，不是表达式
    let x = 5;
    // 2.右边'x+1'就是一个表达式，返回6绑定到y变量上
    let y = x + 1;
    println!("demo2：x={},y={}",x,y);
    // 3.花括号'{}'也是一个表达式
    let z = {
      let a = 3;
        a+1 // 后面不能有";"，不然"a+1"就是语句了就没有返回值了
    };
    println!("demo2：z={}",z)
}

// 3.函数返回值
fn demo3() -> i16{
    let x = 8;
    4+x // 后面不能带"；"，不能写成"4+x;"，否则就是语句就没有返回值了,会报错："implicitly returns `()` as its body has no tail or `return` expression"
    // 等价于下面写法
    //return 4+x
    //等价于下面写法,
    //return 4+x;
}