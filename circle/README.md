Rust提供了3种循环工具,分别是：for,while,loop。<br/>
一个循环会执行循环体中的代码直到结尾，并紧接着回到开头继续执行。
### for循环
for循环也是大多数语句都支持的，
```go
// for循环
fn demo2() {
    let arr = [1, 2, 3, 4, 5, 6];
    for element in arr.iter() {
        println!("demo2输出，element:{}", element)
    }
}
```
### while循环
while循环是在每次执行循环体之前都判断一次条件，假如条件为真则执行代码片段，假如条件为假或在执行过程中碰到break就退出当前循环。这种模式可以通过loop、if、else及break
关键字的组合使用来实现。go语言的while循环也是一样的用法，详见下面demo1示例：
```go
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
```

### loop循环