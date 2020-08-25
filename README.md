# rust-study
rust 学习笔记

本仓库主要记录一些自己学习rust的过程，以官方的《The Rust Programming Language》为主

## rustup & rustc & cargo

- rustup是rust的版本和相关的工具的命令行工具，类似于nvm
- rustc是rust的编译器，对rust代码进行编译
- cargo是rust的包管理器，类似于npm、yarn

## ch1

- cargo build 编译cargo初始化项目
- cargo run 编译并运行直接产生的可执行文件
- cargo check 只对rust代码进行check，而不生成可执行文件。在开发阶段可以大大减少时间

## ch2

- let mut x & let x

声明 mut 代表的是可变数据， 不声明就代表这个变量是不可变的

- std::io::stdin

std::io::stdin 返回一个std::io::Stdin的实例,可以当做你在终端进行的标准输入的一种处理

- &
获取对一个变量的引用