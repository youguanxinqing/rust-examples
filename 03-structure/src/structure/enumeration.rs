/// 枚举类型: 创建一个从数个不同取值中选其一的枚举类型（enumeration）
///

#[derive(PartialEq)]
enum WebEvent {
    // 单元结构体
    PageLoad,
    PageUnLoad,
    // 元组结构体
    KeyPress(char),
    Paste(String),
    // 普通结构体
    Click{x: i64, y: i64}
}

fn use_enum_type() {
    fn inspect(event: WebEvent) {
        use super::*;

        match event {
            WebEvent::PageLoad => println!("PageLoad"),
            WebEvent::PageUnLoad => println!("PageUnLoad"),
            WebEvent::KeyPress(c) => println!("KeyPress, c: {}", c),
            WebEvent::Paste(s) => println!("Paste, s: {}", s),
            WebEvent::Click {x, y} => println!("Click, x: {}, y: {}", x, y),
        }
    }
    inspect(WebEvent::Click {x: 512, y: 18});
    inspect(WebEvent::KeyPress('f'));

    type Event = WebEvent;  // 类型别名
    let add = Event::PageLoad;
    if add == Event::PageLoad {
        println!("PageLoad 1");
    }
}

fn use_type_alias() {
    type Event = WebEvent;  // 类型别名
    let add = Event::PageLoad;
    if add == Event::PageLoad {
        println!("PageLoad 1");
    }

    // 最常见的别名是 impl 块中的 Self
    impl WebEvent {
        fn run(&self) {
           match self {
               Self::PageLoad => println!("PageLoad 2"),
                _ => println!("other"),
           }
        }
    }

    let e = WebEvent::PageLoad;
    e.run();
}

fn use_use_keyword() {
    // 第一种用法: 用 use 可以避免使用完整路径
    use WebEvent::{PageLoad, PageUnLoad};
    println!("{}", WebEvent::PageLoad == PageLoad);  // true

    // 第二种用法:
    // use WebEvent::*;
}

fn use_c_style_enum() {
    // 显式辨别值
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
    }
    // 将 enum 转成整型
    println!("Color::Red -> {}", Color::Red as i32);

    // 隐式辨别值
    enum Number {
        Zero,
        One,
    }
    // 将 enum 转成整型
    println!("Number::One -> {}", Number::One as i32);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_enum_type() {
        use_enum_type()
    }

    #[test]
    fn test_use_type_alias() {
        use_type_alias()
    }

    #[test]
    fn test_use_use_keyword() {
        use_use_keyword()
    }

    #[test]
    fn test_use_c_style_enum() {
        use_c_style_enum()
    }
}