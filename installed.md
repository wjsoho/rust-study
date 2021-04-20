### 一、在Linux或macOS环境中安装`Rust`
安装过程也是非常简单的，打开命令行终端，并且输入命令：</br>
 ```
 curl https://sh.rustup.rs -sSf | sh
 ```

这是下载并通过脚本来安装`rustup`工具，进而安装最新的Rust稳定版本。该脚本可能会在执行过程中请求输入你的密码。
一旦安装成功，你将能够看到如下所示的输出：</br>
```
Rust is installed now. Great!
```
校验是否正确安装</br>
输入命令：`rustc --version`</br>
输出显示：`rustc 1.50.0 (cb75ad5db 2021-02-10)`</br>
通常显示格式依次是：最新稳定版本的版本号、当前版本的哈希码、版本的提交日期： `rustc x.y.z (abcabcabc yyyy-mm-dd)`

### 二、更新与卸载
成功地安装了`Rust`后，如果要更新Rust版本，可以使用如下命令：
```
rustup update
```
如果要卸载rustup及Rust工具链，可以使用如下命令：
```
rustup self uninstall
```

### 三、输出`hello world`

1.创建一个`projects/hello_world文件夹`，通常文件夹下划线方式命名。
```
$ mkdir ～/projects
$ cd ～/projects
$ mkdir hello_world
$ cd hello_world
```
2.创建一个`main.rs`文件，并输入以下内容：
`main()`是主函数，习惯用4个空格隔开而不是`tab`键，用`；`结尾表示一行结束下一行开始。
```
fn main() {
    println!("Hello, world!");
}
```
3.编译与运行，编译后会在当前目录下生成一个二进制的可执行文件
```
$ rustc main.rs
$ ./main
Hello, world!
```
