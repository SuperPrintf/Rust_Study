<div align="center">

# **猜数字游戏**</div>

## **1.项目内容**
* [_`请求玩家对这个数字进行猜测`_](https://github.com/SuperPrintf/Rust_Study/tree/main/2.initial#处理一次猜测)<br>
* [_`生成一个1-100的随机数`_](https://github.com/SuperPrintf/Rust_Study/tree/main/2.initial#生成一个随机数字)<br>
* [_`假如输入数字与随机数不同,那么将会给出数字偏大或者偏小的提示`_](https://github.com/SuperPrintf/Rust_Study/tree/main/2.initial#判断大小)<br>
* [_`假如玩家猜对了数字,则程序会打印一段祝贺并退出`_](https://github.com/SuperPrintf/Rust_Study/tree/main/2.initial#条件退出)<br>

## **2.代码实现**
* ### 处理一次猜测<br>
    * 文件为src/main.rs
    ```rust
    use std::io;

    fn main() {
        println!("Guess the number!");

        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
    }
    ```
    1. 导入`std::io`库,以实现输入获取<br>
    ```rust
    use std::io; //该行代码放在首行
    ```
    >* _std::io不是`预导入`(prelude)的条目_<br>
    >* _需要用use语句显式的进行导入声明_<br>
    2. 输出提示<br>
    ```rust
    println!("Guess the number!");
    println!("Please input your guess:");
    ```
    3.创建存储空间<br>
    ```rust
    let mut guess = String::new();
    ```
    >* _`let`语句创建了一个新的`变量`(variable)_<br>
    >* _`mut`关键字修饰变量表示其可变_<br>
    >* _`=`将右值绑定至左边变量_<br>
    >* _`String::new`函数创建了一个新的空白字符串_<br>
    >* _`::`表明new是String类型的一个`关联函数`(associated function)/`静态方法`(static method)_<br>
    4. 输入获取<br>
    ```rust
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("Your guess is: {guess}");
    ```
    >* `stdin()`函数返回一个`std::io::Stdin`的实例,这代表终端标准输入句柄的类型<br>
    >* .read_line(),调用`read_line`方法从标准输入句柄获取用户输入<br>
    >* `&`表示这个参数是一个`引用`(reference),它允许多处代码访问同一处数据,而无需在内存中多次拷贝<br>
    >* 通过Result进行异常处理:
    >>* `read_line`会将用户输入附加到传递给它的字符串中,不过它也会返回一个类型为`Result`的值<br>
    >>* `Result`是一种枚`举类型`(enum),成员是`Ok`和`Err`<br>
    >>* `Ok`成员表示操作成功,内部包含成功时产生的值<br>
    >>* `Err`成员则意味着操作失败,并且包含失败的前因后果<br>
    >>* `Result`类型的值像其他类型一样,拥有定义于其上的方法,`Result`的实例拥有`expect`方法<br>
    >>* 如果`io::Result`实例的值是`Err`,`expect`会导致程序崩溃,并显示当做参数传递给`expect`的信息<br>
    >>* 如果`Result`实例的值是`Ok`,`expect`会获取`Ok`中的值并原样返回<br>
    >>* 如果不接收`read_line()`的返回值,则在编译时Rust会警告没有使用`read_line`的返回值`Result`,说明有一个可能的错误没有处理<br>
    >* 关于`println!("Your guess is: {guess}");`中的占位符`{}`<br>
    >>* `{}`可以使用多个,与格式化字符一一对应
    >>* 格式化字符除了直接写入`{}`外还可使用如下格式
    >>* println!("Your guess is: {}", guess);
* ### 生成一个随机数字<br>
    1. 导入依赖包以获取更多功能,首先在.toml文件声明依赖包及其版本<br>
    2. 通过在Cargo.toml文件中添加如下描述<br>
    ```toml
    [dependencies]

    rand = "0.3.14"
    ```
    3. 关于依赖包版本的查看<br>
        * 可以在此处限定包版本为指定版本,可有效防止出现过高或过低版本出现依赖包不兼容的情况<br>
        * 当前(2023.01)rand包的最高版本为V0.8.5<br>
        * 本程序的调用在V0.7.3及以下版本有效<br>
        * 推荐安装[`Better TOML`](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml "拓展主页")拓展
        * 在VS Code或CLion中编辑.toml文件时可通过指针悬停的方式查看依赖包的历史版本及快速跳转说明文件<br>
        ![通过VS Code开启Cargo文件可以查看以来包版本信息](https://p.ipic.vip/ac6q7j.png "鼠标悬弹出下拉栏显示历史版本")<br>
* ### 判断大小<br>
* ### 条件退出<br>
