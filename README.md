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

>声明 mut 代表的是可变数据， 不声明就代表这个变量是不可变的

- std::io::stdin

>std::io::stdin 返回一个std::io::Stdin的实例,可以当做你在终端进行的标准输入的一种处理

- &
>获取对一个变量的引用

- variable with mut VS shadowing

>variable with mut可以改变一个变量的值，但是改变这个变量的类型的时候会报错

shadowing 可以使用之前的变量名并且可以改变值和类型

### 数据类型

1. rust含有4种数据类型：整数，浮点数，布尔型，字符串
- 整数类型
  - i8(-127~128) u8(0, 255)
  - i16 u16
  - i32(default) u32
  - i64 u64
  - i128 u128
  - isize usize
- 浮点数类型
  - f32 f64
- 布尔型
2. 复合类型
- 元组
> 不可以扩展和缩小
```rust
fn main(){
  let tup = (500, 6.4, 1);
  //解构
  let (x, y, z) = tup;

  let mut num = tup.0
}
```
- 数组
```rust
let arr: [i32; 5] = [1,2,3,4,5];

let a = [3;5] // let a = [3,3,3,3,3]
```
- 函数
```rust
fn main(){
  let x = 5;

  let y = {
    let x = 3;
    x + 1
  };

  let res = plus_one(y)

  println!("The value of res is {}", res)
}

fn plus_one(x: i32) -> i32{
  x + 1
}
```

## ch4 

### ownership

- rust中的每一个值都有一个变量作为它的所有者
- 每个值一次只可以有一个所有者
- 当所有者离开作用域，那个值将会被释放

### reference（without ownership）

- 在任何时候，你只能有一个mutable reference 或 任意数量的 immutable references
- 引用必须总是有效的， 避免出现野指针的情况

### slice (without ownership)

- string文本的类型 是 string slice后的引用。
> This is also why string literals are immutable; &str is an immutable reference.

## ch5 Struct

结构体类似于JS中的Object

在impl中定义的方法可以使用&self获取本身的引用

## ch6 enum
enum 一种枚举类型，可以包含不同基本类型，包括Struct

enum与struct都支持impl

>有用的枚举类：Option
 ```rust
 enum Option<T>{
   Some<T>,
   None
 }
 ```

enum 类型的值 只可以通过match 和 if let来获得

## ch7 mod

mod 声明模块

pub 声明公有

use 使用模块

pub use 则外部文件也可以访问到模块

```rust
use std::io as IoResult; //别名

use std::{io, com::Ordering};

use std::io::{self, Write};

use std::collections::*;
```


## ch7 common collections(**Vectors , string & hash map**)

### vectors
```rust
// initial
let v:Vec<i32> = Vec::new()

let v = vec![1,2,3];

// update
let mut v = Vec::new();

v.push(5);
v.push(6);

// vectors 同样受所有权限制

let first = &v[0] // not allowed, vec也是引用类型，不可同时出现mutable and immutable
```

### String
> String  is a growable, mutable, owned, UTF-8 encoded string type

```rust
let string = "Hello world";

let s = data.to_string();

//or

let s = String::from("Hello world");

// update string

let mut s = Stringf::from("foo");

s.push_str("bar");

let s3 = String::from("tic");
let s4 = String::from("tac");
let s5 = String::from("toe");

let res = format!("{}-{}-{}", s3,s4,s5); // does not take ownership~
println!("res is {}, s3 is {}", res, s3);

// String 不支持index索引
1.String is a wapper of Vec<u8>, 转换出来的UTF-8转换的值，所以使用索引是没有意义的
2.时间复杂度建议是O(1), 但是String的话是rust必须计算从初始到index的有多少个有效的字符

// Hashmap

let text = "cl cll cl ll";

let mut map = HashMap::new();

for word in text.split_whitespace(){
    let count = map.entry(word).or_insert(0);
    // or_insert returns a &mut V
    *count += 1;
}

println!("{:?}", map);
// {"cl": 2, "cll": 1, "ll": 1}


```



