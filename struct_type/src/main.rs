fn main() {
    demo1()
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 定义一个结构体
fn demo1() {
    let user = User {
        username: String::from("A"),
        email: String::from("a@163.com"),
        sign_in_count:1,
        active:false,
    };
    print!("user:{:?}",user)
}


/*
struct User {
username: String,
email: String,
sign_in_count: u64,
active: bool,
}
*/