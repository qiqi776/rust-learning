// fn main() {
//     println!("Hello, world!");
//     {
//         let x = 5;              // ----------+-- 'b
//                                 //           |
//         let r = &x;             // --+-- 'a  |
//                                 //   |       |
//         println!("r: {}", r);   //   |       |
//                                 // --+       |
//     }                           // ----------+
    // &i32        // 一个引用
    // &'a i32     // 具有显式生命周期的引用
    // &'a mut i32 // 具有显式生命周期的可变引用
// }

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    test1();

    test3("abc", "def");

    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}",i);
    // 错误的例子
    // let i;
    // {
    //     let novel = String::from("Call me Ishmael. Some years ago...");
    //     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //     i = ImportantExcerpt {
    //         part: first_sentence,
    //     };
    // }
    // println!("{:?}",i);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 'a的生命周期是参数作用域较小的那个
// 这里的生命周期'a和string2的生命周期'a是一样的，因为它是作用域较小的那个
fn test1() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
// 会报错
// fn test2<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// 可以依靠返回所有权来解决这个问题
fn test3(_x: &str, _y: &str) -> String {
    String::from("really long string")
}

// struct中的生命周期
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 方法中的生命周期

// 泛型语法
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// 方法的声明周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
