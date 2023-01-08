//fn声明一个函数,这里是一个名为main,没有入参和返回值的函数
fn main(){ //main()是Rust的入口函数
    println!("Hello world!"); //这里并非一个普通函数,而是名为println!的宏(格式化输出并换行)
    print!("Helklo Rustc\n"); //Rust中所有以!结尾的都意味着是宏而非函数
}//{}用以标记函数体,Rust要求所有函数体都要被{}阔起
