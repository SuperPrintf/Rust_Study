**构建和编译Rust文件**
=
**1.文件结构**
--
* Rust文件的后缀名为.rs<br>
* 若使用rustc命令进行编译,则无需任何文件结构可直接操作<br>
* 若使用cargo进行编译,需经历新建、构建、执行共三个步骤<br>
* 通过Mac终端命令执行操作<br>  
**2.环境配置**
--
* 安装Rust<br>
    * MacOS/Linux<br>
        执行安装脚本
        ```bash
        curl https://sh.rustup.rs -sSf | sh
        ```<br>
        
        让配置立即生效
        ```bash
        source $HOME/.cargo/env
        ```<br>
        
    * Windows
        参见[Rust官网教程](https://www.rust-lang.org/tools/install)<br>
* 更新与卸载<br>
    版本查看
    ```bash
    rustc --version
    ```<br>
    
    更新Rust版本
    ```bash
    rustup update
    ```<br>
    
    卸载Rust
    ```bash
    rustup self uninstall
    ```<br>
    
* IDE配置
    * VS Code插件推荐
        `rust-analyzer`获得Rust语言支持(必要)<br>
        `Rust Syntax`改进的语法高亮<br>
        `GitHub Copilot`注释补全(付费)<br>
        `Tabnine AI Autocomplete`自动推断补全<br>
    * CLion插件推荐
        `Rust`获得Rust语言支持(必要)<br>
        `GitHub Copilot`注释补全(付费)<br>
        `Tabnine AI Autocomplete`自动推断补全<br>
**3.命令操作**
--
* rustc的编译操作<br>
    * 创建文件
    ```bash
    mkdir hello_rustc
    cd hello_rustc
    touch hello.rs
    ```<br>
    
    * 编写并编译
    ```bash
    vim hello.rs
    rustc hello.rs -o hello
    ./hello
    ```<br>
    
* 通过cargo的构建及编译<br>
    * 创建工程
    ```bash
    cargo new hello_cargo
    ```<br>
    
    * 项目构建
    ```bash
    cd hello_cargo
    cargo build
    ```<br>
    
    构建产生的可执行文件会被存放在target/debug中<br>
    此时项目文件已经构建完毕,后续可用vim编辑src中的.rs文件或者直接使用IDE进行编辑调试<br>
    
    * 通过VS Code开启文件
    ```bash
    open -a /Applelications/Visual\ Studio\ Code.app ../hello_cargo
    ```<br>
    
    只有构建好的代码才能在VS中Debug<br>
    CLion则可在创建项目时自动完成构建,无需使用cargo命令<br>
    
    * 检查是否能够通过编译
    ```bash
    cargo check
    ```<br>

    可以快速检测代码是否能通过编译,不会生成可执行文件<br>
    
    * 运行项目
    ```bash
    cargo run
    ```  <br>
    
    * 以Release模式进行构建
    ```bash
    cargo build --release
    ```  <br>
    优化模式下生成的代码将会存储在target/release,而非在target/debug中<br>
    在优化模式下产生的代码拥有更小的体积和更高的性能,但相应的编译时会占用更多资源与时间<br>
