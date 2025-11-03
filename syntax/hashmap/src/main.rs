use std::collections::HashMap;
fn main() {
    // 创建一个空的 HashMap
    let mut scores = HashMap::new();
    // 所有的键必须是相同类型，值也必须都是相同类型
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    // 把vector转换为HashMap
    let vec = vec![("key1","value1"), ("key2","value2")];
    let map: HashMap<_,_> = vec.into_iter().collect();


    // 访问hashmap里面的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // 遍历KV
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 所有权
    let field_name = String::from("favoutite color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name和field_value被移动到map里面，不能再被使用了
    // println!("{}: {}", field_name, field_value); // 编译错误


    // 更新hashmap
    // key 存在：替换还是保留？
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores); // {"Blue": 25}


    // 使用 entry 方法只在键没有对应一个值时插入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");


    // 基于key原来的值进行更新
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    // Hashing函数，默认使用SipHash算法


    // 练习1:能否通过编译
    // let mut h = HashMap::new();
    // h.insert("k1", 0);
    // let v1 = &h["k1"];
    // h.insert("k2", 4);
    // let v2 = &h["k2"];
    // println!("v1: {}, v2: {}", v1, v2); // v1: 0, v2: 4

    // 练习2：能否通过编译
    let mut h: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in "hello".chars().enumerate() {
        h.entry(c).or_insert(Vec::new()).push(i);
    }
    let mut sum = 0;
    for i in h.get(&'l').unwrap() {
        sum += *i;
    }
    println!("sum: {}", sum);
}
