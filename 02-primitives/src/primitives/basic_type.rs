use std::mem;

// 打印参数类型
fn type_name<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn use_basic_type() {
    // 默认情况下, 浮点数f64, 整数i1
    let f1 = 5.12;
    let i1 = 10;
    println!("{} type is {}; {} type is {}", "f1", type_name(&f1), "i1", type_name(&i1));

    let num1 = 10i64;
    let num2 = 10 + num1;  // 根据上下文推断 num2 为 i64
    println!("{} type is {}; {} type is {}", "num1", type_name(&num1), "num2", type_name(&num2));

    let x = ();  // 单元类型
    println!("{} type is {}", "x", type_name(&x));

    let arr = [1, 2, 3];  // 数组: [i32; 3]
    let tup = (1, 2, true); // 元组: (i32, i32, bool)
    println!("{} type is {}; {} type is {}", "arr", type_name(&arr), "tup", type_name(&tup));
}

fn use_tuple_type() {
    // 元组支持各种类型
    let t1 = (1, true, "zhong", 3.14, (1, 1.23));
    println!("t1: {:?}", t1);

    // 单个元组需要","
    let t2 = (10,);
    println!("t2: {:?}", t2);

    // 不支持太长的元组
    // 目前最大支持 12 个元素
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

    let t3 = (1, 2, true);
    let (item1, item2, item3) = t3;  // 解构元组
    println!("item1:{}, item2:{}, item3:{}", item1, item2, item3);
}

fn use_array_and_slice()  {
    // 数组类型: [T; length], 编译时确定
    // 切片类型: &[T], 运行时确定

    // 初始化 5 个 1
    let arr1 = [1; 5];
    // 声明赋值
    let mut arr2 = [1, 2, 3, 4, 5];
    println!("arr1: {:?}; arr2: {:?}", arr1, arr2);

    // 数组转切片
    let mut s1 = &mut arr2[1..3];  // arr2在这里会发生借用
    println!("s1: {:?}", s1);

    fn analyze_slice(slice: &[i32]) {
        println!("slice: {:?}, size is: {}", slice, mem::size_of_val(&slice))
    }
    let t3 = [1, 2, 3, 4, 5];
    println!("t3: {:?}, size is: {}", t3, mem::size_of_val(&t3));
    analyze_slice(&t3);  // 数组可以**隐式**转切片

    // println!("{}", t3[5]);  // 数组越界, **编译期**报错
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_basic_type() {
        use_basic_type()
    }

    #[test]
    fn test_use_tuple_type() {
        use_tuple_type()
    }

    #[test]
    fn test_use_array_and_slice() {
        use_array_and_slice()
    }
}