
fn print_hello_world() {
    println!("hello world");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 打印 hello world
    #[test]
    fn test_print_hello_world() {
        print_hello_world();
    }
}