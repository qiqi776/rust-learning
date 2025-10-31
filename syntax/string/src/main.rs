fn main() {
    let mut s = String::new();

    let data = "initial contents"; // 字符串字面值
    let s = data.to_string(); // 从字符串字面值创建String

    let s = "initial contents".to_string(); // 直接调用to_string方法

    let s = String::from("initial contents"); // 使用String::from函数创建String


    // 字符串都是UTF-8编码的，因此可以包含任何有效的Unicode字符。
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // 更新字符串
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); // 在末尾添加字符串
    s.push('!'); // 在末尾添加单个字符
    println!("{}", s); // "foobar!"

    let mut s = String::from("lo");
    s.push('l'); // 现在s是"lol"
    println!("{s}");

    // 字符串拼接
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // + 使用的是add方法，需要将s1移动到堆上，因此不能再使用
    // 实际上是获取了s1的所有权，并将s2附加到了s3上
    let s3 = s1 + &s2;
        
    // 问题1：使用A+B和A.push_str(B)进行字符串拼接，有什么区别？
    // 答案1：A+B会获取A的所有权，而A.push_str(B)不会获取所有权。



    // 使用format!宏进行字符串拼接，更加方便且不获取所有权
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    
    // 问题2：let s = s1 + "-" + &s2 + "-" + &s3; 这样写最多可能发生多少次heap分配
    // 答案2：最多可能发生7次heap分配



    // 对String进行索引操作
    // rust 不允许对String进行索引操作，因为字符串是UTF-8编码的，直接索引可能会导致无效的字符边界
    let s1 = String::from("hello");
    // let h = s1[0]; // 编译错误

    // 三种与string相关的视角:字节、标量值（char）、字形簇（grapheme cluster）
    // 索引操作预期总是需要常数时间（O(1)）但是对于 String 不可能保证这样的性能
    // 因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符


    // 对string进行切片(如果真的需要索引创建string切片，必须更加具体)
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 切片操作返回一个新的&str
    println!("{s}"); // 输出 "Зд"

    // 获得单个Unicode标量值（char）
    for c in "Зд".chars() {
        println!("{c}");
    }
    // 获得字符串的字节表示
    for b in "Зд".bytes() {
        println!("{b}");
    }
}

// 问题3：为什么Rust不支持对String进行索引操作？
// 答案3：因为字符串是UTF-8编码的，字符串索引不明确，因为字符串表示多个粒度的顺序数据

// 问题4：字符串切片&str和字节切片&[u8]有什么区别？
// 答案4：&str是字符串切片，表示一段有效的UTF-8编码的文本，而&[u8]是字节切片，表示一段原始的字节序列。