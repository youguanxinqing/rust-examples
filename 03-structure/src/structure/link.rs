use std::fmt::format;
use crate::structure::link::List::Cons;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        return Self::Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // 匹配一个具体的 `T` 类型要好过匹配引用 `&T`
        match *self {
            // self 是借用来的, 所以不能获取 tail 的所有权. 需要用 `ref` 获取引用
            Self::Cons(_, ref tail) => 1 + tail.len(),
            Self::Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Self::Cons(head, ref tail) => {
                format!("{} -> {}", head.to_string(), tail.stringify())
            } ,
            Self::Nil => "nil".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::link::List;

    #[test]
    fn test_use_list() {
        let mut l = List::new();
        l = l.prepend(1);
        l = l.prepend(10);
        l = l.prepend(3);
        println!("{}", l.stringify());
        println!("len: {}", l.len());
    }
}