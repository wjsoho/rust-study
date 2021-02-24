// 导入包
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    // 生成一个随机数
    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("this secret number is {}", secret_number);
    // loop循环执行知道猜中为止
    loop {
        println!("Please input your guess.");
        // 声明一个可变变量guess
        let mut guess = String::new();
        // 从键盘读取输入的字符存入guess变量中
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // 解析guess变量，如果不是数字字符串，重新输入直到正确为止
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please type a number");
                continue;
            }
        };
        println!("You guessed: {}", guess);
        // 比较大小，直到相等才break停止循环
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}