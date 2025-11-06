use std::cmp::PartialOrd; // 导入 PartialOrd trait

// 1. 为largest添加 T: PartialOrd 的 trait 约束
// 这告诉编译器：T 必须是可以使用 > 运算符进行比较的类型
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<X, Y> {
    x: X,
    y: Y,
}

// 2. 为 Point<X, Y> 实现方法,所有的 Point方法都放在这一个 impl 块中
impl<X, Y> Point<X, Y> {
    // 这个方法适用于任何 Point<X, Y>
    fn x(&self) -> &X {
        &self.x
    }

    // mixup方法演示了更复杂的泛型
    // 它可以接收一个不同泛型参数的 Point
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // --- largest 函数 ---
    // 现在可以编译了，因为 i32 和 char 都实现了 PartialOrd
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    // --- Point 结构体 (T, T) ---
    // 即使 x 和 y 类型相同，也适用于 Point<X, Y>
    // Rust 会推断 X=i32, Y=i32
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 }; // X=f64, Y=f64

    // --- Point.x() 方法 ---
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x()); // p.x() 正常工作

    // --- Point.mixup() 方法 ---
    // 演示 Point<X, Y> 的灵活性
    let p1 = Point { x: 5, y: 10.4 };      // p1 类型: Point<i32, f64>
    let p2 = Point { x: "Hello", y: 'c' }; // p2 类型: Point<&str, char>

    // p1 (X1=i32, Y1=f64)
    // p2 (X2=&str, Y2=char)
    // 结果 p3 的类型是 Point<X1, Y2> => Point<i32, char>
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}