use std::any::type_name;

fn type_of<T>(_: &T) -> String {
    format!("{}", type_name::<T>())
}

fn use_unused_variable() {
    let unused_variable = 10;

    // 用下划线避免未使用变量告警
    let _unused_variable = 10;
}

fn use_shadow_variables() {
    let a = 12;
    println!("type: {}, val: {}", type_of(&a), a);

    {
        // 1. 遮蔽 a 变量
        let a = "zhong";
        println!("type: {}, val: {}", type_of(&a), a);
    }

    // 2. 遮蔽 a 变量
    let a = 3.123;
    println!("type: {}, val: {}", type_of(&a), a);
}

fn use_freeze_variable() {
    // 1. 创建一个可修改的变量
    let mut a = 10;
    {
        let a = a;
        // a = 11;  // error, 被不可变变量绑定, 使得 a 不能被修改, 也称为冻结.
    }
    a = 12;  // 离开作用域后就可以正常修改了.
    println!("{}", a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_unused_variable() {
        use_unused_variable()
    }

    #[test]
    fn test_use_shadow_variables() {
        use_shadow_variables()
    }

    #[test]
    fn test_use_freeze_variable() {
        use_freeze_variable()
    }
}