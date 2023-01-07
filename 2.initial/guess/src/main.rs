//生成一个随机数让用户猜测该随机数的值

use std::io; //将标准库(std)中的io模块引入本作用域
use std::cmp::Ordering;  //导入标准库中cmp的Ordering枚举类
use rand::Rng; //Rng是一个特征(trait),定义了随机数生成器需要实现的方法的集合

fn main() {
    println!("Guess the number!");

    //rand::thread_rng()返回一个特定的随机数生成器
    //随机数生成方法.gen_range()生成范围在1-100的随机数
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        //通过let语句创建了一个新变量,Rust中变量默认是不可变的
        //使用mut关键字声明变量表示该变量具有可变性                                                                                                                                              
        //通过等号运算符,变量将与右值绑定
        //String类通过::运算符调用关联函数
        //Rust拥有自动类型推导能力,自动将guess类型推导为String
        let mut guess = String::new();
        println!("Please enter your guess:");

        //stdin()函数将会返回std::io::stdin的实例,它被用做句柄来获取
        //标准输入句柄的read_line()方法获取用户输入
        //read_line()方法需要一个可变变量的引用,将它作为传入数据的存储地址
        //&mut表示一个可变的引用
        //read_line()方法会返回一个枚举类型的io::Result
        //expect()方法的调用可以在出现错误时直接退出
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line\n");

        //Rust允许使用同名新变量来隐藏(shadow)旧变量的值
        //trim方法将会去掉String首位空白的字符(输入时的'\n'被带入了字符串)
        //parse方法会尝试将字符串转换为数字
        //:u32表示手动为变量指定类型为无符号整型
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
        println!("Your guess is: {}", guess);

        //cmp方法能够为任何可比较的值类型计算出他们比较之后的结果
        //cmp方法会返回一个引入作用域的Ordering枚举类型的变体
        //match表达式由数个分支(arm)组成
        //match每个分支都有一个用于匹配的模式(pattern)以及匹配成功后要执行的相应代码
        //match将会把传入参数依次与分支进行比较
        match guess.cmp(& secret_number) {
            Ordering::Less => println!("Too samall!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;//break关键字退出loop循环
            }
        }
    }
}