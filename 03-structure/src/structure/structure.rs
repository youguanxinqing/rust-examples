/// 三种结构体：
/// 1. 元组结构体 (具名元组)
/// 2. C struct
/// 3. 单元结构体 unit struct
///

fn use_tuple_struct() {
    #[derive(Debug)]
    struct Student(String, i32);

    let s1 = Student("zhong".to_string(), 18);
    println!("{:?}", s1);

    // 解构元组结构体
    let Student(name, age) = s1;
    println!("name2: {}, age2: {}", name, age);
}

fn use_c_struct() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Student {
        name: String,
        age: i32,
    }
    let s1 = Student{name: "zhong".to_string(), age: 18};
    println!("{:?}", s1);

    // 解构 C 结构体
    let Student{
        name, age
    } = s1;
    println!("name: {}, age: {}", name, age);
}

fn use_unit_struct() {
    // 单元结构体
    struct Unit;
    // 实例化单元结构体
    let _unit = Unit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_tuple_struct() {
        use_tuple_struct()
    }

    #[test]
    fn test_use_c_struct() {
        use_c_struct()
    }

    #[test]
    fn test_use_unit_struct() {
        use_unit_struct()
    }
}
