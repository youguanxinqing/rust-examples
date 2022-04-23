/// format!, print!, println, eprint!, eprintln!: 格式化的正确性编译器确定

fn print_format_string() {
    println!("{} days", 31);

    // 索引参数
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 命名参数
    println!("{subject} {verb} {object}", object="you", verb="love", subject="I");

    // ":" 后面指定格式
    // :b 二进制输出格式: 2 -- output --> 10
    println!("{} of {:b} people know binary, the other half don't.", 1, 2);

    // 宽度对齐, 默认用空格
    println!("{number:>width$}", number=10, width=6);
    // 宽度对齐, 使用"0"
    println!("{number:>0width$}", number=10, width=6);

    // error: 参数不对, 编译期失败
    // println!("My name is {0}, {1} {0}", "Bond");

    #[derive(Debug)]
    struct Structure(i32);
    let s = Structure(512);
    println!("{:?}, {}", s, s.0);
}

fn impl_fmt_display() {

    struct Structure(i32, String);

    // 实现 Display trait
    impl std::fmt::Display for Structure {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(format!("S<Display>: {} {}", self.0, self.1).as_str())
        }
    }
    // 实现 Debug trait
    impl std::fmt::Debug for Structure {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("")
                .field(&self.0)
                .field(&self.1)
                .finish()
        }
    }

    println!("{}", Structure(512, "zhong".to_string()));
    println!("{:?}", Structure(512, "guan".to_string()));
}

fn print_float() {
    let pi = 3.1415926;
    // 小数点后三位
    println!("{:.3}", pi);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_format_strings() {
        print_format_string();
    }

    #[test]
    fn test_impl_fmt_display() {
        impl_fmt_display();
    }

    #[test]
    fn test_print_float() {
        print_float();
    }
}