extern "Rust" {
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32;
    #[no_mangle]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod foo {
    // 不需要 #[no_mangle]，它会自动使用 Rust 的默认符号名处理
    #[no_mangle]
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }

    // 同样，定义 my_demo_function_alias 为 my_demo_function 的别名
    #[no_mangle]
    pub fn my_demo_function_alias(a: u32) -> u32 {
        my_demo_function(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // SAFETY: 我们知道这些函数是 Rust 函数的别名
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
