use std::slice::Iter;
use std::ops::Range;

fn main() {
    // 创建空的Vector
    let _v1: Vec<i32> = Vec::new();
    // 创建有值的Vector
    let v2 = vec![1, 2, 3, 4, 5];

    // 使用push方法添加元素
    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);

    // 使用索引访问元素
    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    // 使用get方法访问元素
    let third:Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    // 索引越界
    // let not_exist = v2.get(100);
    // let not_exist2 = &v2[100]; // 这行代码会引发panic

    // 遍历Vector
    let mut v = vec![100, 32, 57];
    for n_ref in &v {
        let n_plus_one = *n_ref + 1;
        println!("{}", n_plus_one);
    }
    for n_ref in &mut v {
        *n_ref += 200;
        println!("{}", n_ref);
    }

    // 迭代器
    let v4:Vec<i32>         = vec![1, 2];
    let mut iter: Iter<'_, i32> = v4.iter(); // iter指向v4的第一个元素
    let _n1: &i32                = iter.next().unwrap(); // n1指向v4的第一个元素1
    let _n2: &i32                = iter.next().unwrap(); // n2指向v4的第二个元素2
    let _end: Option<&i32>       = iter.next(); // end指向None，因为v4只有两个元素

    let mut iter: Range<usize>  = 0 .. v4.len(); // 创建一个范围迭代器，范围是0到v4的长度
    let i1: usize               = iter.next().unwrap(); // i1是0
    let _n1: &i32                = &v4[i1]; // n1是v4的第一个元素1


    // 使用枚举存储不同类型的数据
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    // 使用迭代器遍历Vector并复制元素
    let v5 = dup_idiomatic(&v);
    println!("v: {:?}, v5: {:?}", v, v5);
}

// 使用迭代器遍历Vector,在这里push会导致编译错误
// fn dup_in_place(v: &mut Vec<i32>) {
//     // 1. v.iter() 创建了一个对 v 的“不可变借用” (&v), 这个借用会持续到 'for' 循环结束
//     for n_ref in v.iter() { 
//         // 2. v.push() 试图创建一个对 v 的“可变借用” (&mut v)
//         v.push(*n_ref);
//     }
// }
// 正确的遍历Vector的方法
#[allow(dead_code)]
fn dup(v: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for n in v {
        // 创建了对 result 的“可变借用” (&mut result)
        result.push(*n);
    }
    result
}
// 我们把它改成更推荐的"Rust 风格" 的写法
fn dup_idiomatic(v: &Vec<i32>) -> Vec<i32> {
    // .iter()     -> 获取一个迭代器，元素类型是 &i32
    // .map(|&n| n) -> 将每个 &i32 类型的元素 "解引用" 为 i32 类型
    // .collect()  -> 把所有 i32 元素收集到一个新的 Vec<i32> 中
    v.iter().map(|&n| n).collect()
    
    // 或者，因为 i32 是Copy类型，你可以用 .cloned()，更简洁：
    // v.iter().cloned().collect()
}


// 注意：
// 1. 对于创建空的Vector，使用 Vec::new() 即可。
// 2. 对于创建有值的Vector，使用 vec![] 语法。
// 3. 对于使用索引访问元素，使用 &v[index] 语法。
// 4. 对于使用get方法访问元素，使用 v.get(index) 语法。
// 5. 对于Option<&i32>类型，Some(value)表示有值，None表示无值。
