# *编写一个猜数字游戏*  
## *1.项目内容*   
* 生成一个1-100的随机数<br>
* 请求玩家对这个数字进行猜测<br>
* 假如输入数字与随机数不同,那么将会给出数字偏大或者偏小的提示<br>
* 假如玩家猜对了数字,则程序会打印一段祝贺并退出<br>

## *2.代码实现* 
* 处理一次猜测<br>
    * 导入`std::io`库,以实现输入获取<br>
    ```rust
    usr std::io;
    ```  
    * 输出提示<br>
    ```rust
    println!("Guess the number!");
    println!("Please input your guess:");
    ```  
    * 输入获取<br>
    ```rust
    let mut guess = String::new();
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
    println!("Your guess is: {}", guess);
    ```  
* 生成一个保密数字
