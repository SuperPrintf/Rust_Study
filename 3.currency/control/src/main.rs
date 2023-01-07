//控制流

fn main() {
    /*if表达式*/
    let number = 3;

    //if条件表达式语句根据条件执行不同的分支(arm)
    //代码中的条件必须产生一个bool类型的值,否则会编译报错
    if number < 0 {
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    //Rust不会自动尝试将类型转换为bool类型
    //不过可以用如下语句达到效果
    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;
    //可以使用if else组合条件语句进行多重条件判断
    if number % 4 == 0 {
        println!("number is divisible by 4");
    }else if number % 3 == 0 {
        println!("number is divisible by 3");
    }else if number % 2 == 0 {
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by 4, 3, or 2");
    }

    //在let语句中使用if
    //由于if是一个表达式,所以其可以作为let语句的右值
    //此处number变量的被绑定到了if表达式的输出结果上
    //此外,分支语句产生的返回值必须为同一类型,否则会报错
    let condition = true;
    let number = if condition{
        5
    } else {
        6
    };
    println!("The value of number is {}", number);

    /*循环*/

    /*loop*/
    //使用loop重复执行代码
    //loop可以通过break进行强制退出,且可以带返回值
    let mut counter = 0;
    let result = loop{
        println!("again!");
        counter += 1;

        if counter > 3{
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    /*while*/
    //while条件循环会在条件为真时执行代码块
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");

    /*for*/
    //for循环遍历集合中的每一个元素
    let a = [10, 20, 30, 40, 50, 60];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //for循环生成数字序列
    //通过Range生成从一个数字到另一个数字
    //通过.rev()方法实现序列的翻转
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}
