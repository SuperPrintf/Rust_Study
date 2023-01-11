<div align="center">

# **猜数字游戏**</div>

## **1.项目内容**
* [_`请求玩家对这个数字进行猜测`_](https://github.com/SuperPrintf/Rust_Study/tree/main/2.initial#处理一次猜测)<br>
* [_`生成一个1-100的随机数`_](https://github.com/SuperPrintf/Rust_Study/tree/main/2.initial#生成一个随机数字)<br>
* [_`假如输入数字与随机数不同,那么将会给出数字偏大或者偏小的提示`_](https://github.com/SuperPrintf/Rust_Study/tree/main/2.initial#判断大小)<br>
* [_`假如玩家猜对了数字,则程序会打印一段祝贺并退出`_](https://github.com/SuperPrintf/Rust_Study/tree/main/2.initial#条件退出)<br>

## **2.代码实现**
* ### 处理一次猜测<br>
    1. 导入`std::io`库,以实现输入获取<br>
    ```rust
    usr std::io; //该行代码放在首行
    ```
        * usr表示
    2. 输出提示<br>
    ```rust
    println!("Guess the number!");
    println!("Please input your guess:");
    ```
    3. 输入获取<br>
    ```rust
    let mut guess = String::new();
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
    println!("Your guess is: {}", guess);
    ```
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
