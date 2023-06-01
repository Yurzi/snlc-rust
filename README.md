# SNLC

一个基于Rust过程宏编写的SNL语言（Small Nest Language）的编译器，可以将其SNL语言最终编译为目标代码。该编译器重新设计了部分SNL的文法，但大致一致，除了部分语句对于分号结束符的要求不同。

## 语言功能实现

由于开发周期影响，本实现只完成了SNL语言中的部分功能，并未完全实现其所所有的语言特性。实现的语言功能列举如下。

- [x] 表达式
- [x] 控制语句
- [x] 过程
    - [x] 嵌套函数定义
    - [ ] 递归
    - [ ] 返回值
- [x] 变量声明
- [x] 类型
    - [x] 整形
    - [x] 字符类型
    - [ ] 数组
    - [ ] 类型定义与记录

## 如何使用

通过引入相应的过程宏`snl!`, 即可解析编译其中包括的SNL语言代码，但所有的关键字需要加上`r#`，以防止关键字冲突。可以使用词法分析器较为方便对关键字进行预处理，最后得到相应的文件进行编译。