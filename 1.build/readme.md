<div align="center">

# **构建和编译Rust文件**
</div>

**1.文件结构**
-
* Rust文件的后缀名为.rs<br>
* 若使用rustc命令进行编译,则无需任何文件结构可直接操作<br>
* 若使用cargo进行编译,需经历新建、构建、执行共三个步骤<br>
* 通过Mac终端命令执行操作

**2.环境配置**
-
1. 安装Rust<br>
    * MacOS/Linux<br>
        1. 执行安装脚本
        ```bash
        #MacOS/Linux
        curl https://sh.rustup.rs -sSf | sh
        ```

        2. 让配置立即生效
        ```bash
        #MacOS/Linux
        source $HOME/.cargo/env
        ```

    * Windows
        参见[_Rust官网教程_](https://www.rust-lang.org/tools/install) <br>

2. 更新与卸载<br>
    * 版本查看
    ```bash
    #MacOS/Linux
    rustc --version
    ```

    * 更新Rust版本
    ```bash
    #MacOS/Linux
    rustup update
    ```

    * 卸载Rust
    ```bash
    #MacOS/Linux
    rustup self uninstall
    ```

3. IDE配置
    * VS Code插件推荐<br>
        [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer "获取地址")
        获得Rust语言支持(必要)<br>
        [`Rust Syntax`](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax "获取地址")
        改进的语法高亮<br>
        [`GitHub Copilot`](https://marketplace.visualstudio.com/items?itemName=GitHub.copilot "获取地址")
        注释补全(付费)<br>
        [`Tabnine AI Autocomplete`](https://marketplace.visualstudio.com/items?itemName=TabNine.tabnine-vscode "获取地址")
        AI推断补全<br>

    * CLion插件推荐<br>
        `Rust`获得Rust语言支持(必要)<br>
        `GitHub Copilot`注释补全(付费)<br>
        `Tabnine AI Autocomplete`自动推断补全

**3.命令操作**
-
* rustc的编译操作<br>
    1. 创建文件
    ```bash
    #MacOS/Linux
    mkdir hello_rustc
    cd hello_rustc
    touch hello.rs
    ```

    2. 编写并编译
    ```bash
    #MacOS/Linux
    vim hello.rs
    rustc hello.rs -o hello
    ./hello
    ```

* 通过cargo的构建及编译<br>
    1. 创建工程
    ```bash
    cargo new hello_cargo
    ```

    2. 检查是否能够通过编译
    ```bash
    cargo check
    ```
    >* _可以快速检测代码是否能通过编译,不会生成可执行文件_<br>

    3. 项目构建
    ```bash
    cd hello_cargo
    cargo build
    ```
    <!---!!!
    待解决:***[]***转换中首尾带/或\时无法正常识别的问题
    应该表示为***target/debug/***
    !!!-->
    >* _构建产生的可执行文件会被存放在***target/debug***中_<br>
    >* _此时项目文件已经构建完毕,后续可用vim编辑src中的.rs文件或者直接使用IDE进行编辑调试_<br>

    4. 运行项目
    ```bash
    cargo run
    ```

    5. 以Release模式进行构建
    ```bash
    cargo build --release
    ```
    >* _优化模式下生成的代码将会存储在***target/release***,而非在***target/debug***中_<br>
    >* _在优化模式下产生的代码拥有更小的体积和更高的性能,但相应的编译时会占用更多资源与时间_<br>

    * cargo命令全平台支持,没有命令差异<br>
    * 通过VS Code开启文件:
    ```bash
    #MacOS
    open -a /Applelications/Visual\ Studio\ Code.app ../hello_cargo
    ```
    >* _只有构建好的代码才能在VS中Debug_<br>
    >* _CLion则可在创建项目时自动完成构建,无需使用cargo命令_<br>
