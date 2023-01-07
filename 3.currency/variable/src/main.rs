//数字中间可以加_作为分割
const MAX_POINTS: u32 = 100_000;

fn main() {
    //同名变量可以被新声明的变量所隐藏(shadow),改变所指向的位置
    let x = 5;
    let x = x + 1;
    let x = x + 2;
    println!("The value of x is: {}", x);

    //隐藏机制的意义在于,可以在改变类型的情况下依然对同名变量赋值有效
    let spaces = "    ";
    let spaces = spaces.len();

    /*标量类型(scalar)*/

    /*整数类型*/
    //常见的int类型从i8-u64,可作为数字的后缀以表示类型
    let num_a = 12_0u8; //科学计数表达
    let num_b = 0x120i64; //十六进制表达
    //特殊的,未指定长度size类型,类型长度由平台决定
    let num_c = 0o120usize; //八进制表达
    let num_d = 0b1111_0000isize; //二进制表达
    //特殊的,Byte类型唯一确定u8,无法使用类型后缀
    let num_e = b'A'; //字符

    /*浮点类型*/
    //由于f64与f32并无速度差异,Rust中float类型默认为f64
    let a = 2.0;
    let a: f32 = 3.0;//强制要求亦可使用f32

    /*布尔类型*/
    let t = false;
    let f: bool = true;

    /*字符类型*/
    //Rust中采用单引号区别字符
    //Rust的字符宽度为4byte的Unicode标量,甚至支持emoji作为字符
    let c = 'z';
    let z = 'ƶ';
    let cat = '🐱';

    /*复合类型(compound type)*/

    /*元组(tuple)*/
    //元组拥有固定长度,无法在声明结束后进行更改
    //元组变量被视作单个复合元素,无法直接获取单个的值
    let tup: (i32, f64, u8) = (100, 6.4, 1);

    //通过模式匹配来解构元组,以获取单个值
    let (x, y, z) = tup;
    println!("The value of y is {}", y);
    //亦可通过索引的方式来进行单独访问
    println!("The value 1 is {}", tup.0);

    /*数组类型(array)*/
    //数组中每一个元素都必须是相同的类型
    //数组一旦声明之后长度就不能再改变
    let a = [1, 2, 3, 4];
    //若需创建一个含有相同元素的数组,可以用如下方法
    let b = [3; 5];
    //数组通过下标进行调用
    println!("The number[3] is {}", a[2]);
    
    /*函数(function)*/
    //函数声明放在主函数外,可在其它地方调用
    another_function(20, 30);

    /*语句(statement)和表达式(exoression)*/
    //语句是指执行操作但不反回值的指令
    //表达式指会进行计算并返回值的指令

    //单独的值也是一个表达式其返回自身
    let x = 6;

    //该语句的右值是一个代码块,其作为表达式拥有返回值
    let y = {
        let x = 3;
        x + 5
    };
    println!("The value y is {}", y);


}

//为函数定义参数时,需定义函数的类型
//函数返回值在括号后通过->标明类型
//函数的返回值等同于函数体最后的一个表达式的值
//亦可以使用return来指定返回值
fn another_function(x: i32, y: i32) -> i32 {
    println!("This is Another function,the number is {}.", x);
    println!("Another number y is {}", y);
    x + 1
}
