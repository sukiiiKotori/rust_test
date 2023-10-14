#[allow(unused)]
fn divide(a: i32, b: i32) -> ??? {
    //实现一个除法函数，需要包含对于b为0时的错误处理
    //错误信息为 "Cannot divided by zero"
}

#[allow(unused)]
fn get_square_root(n: f64) -> ??? {
    //实现一个平方根函数，利用标准的sqrt实现，需要包含对于负值的处理
    //负值返回None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(10, 0), Err("Cannot divided by zero".to_string()));
    }

    #[test]
    fn test_get_square_root() {
        assert_eq!(get_square_root(16.0), Some(4.0));
        assert_eq!(get_square_root(-1.0), None);
    }
}