macro_rules! add {
    //编写一个计算两数之和的宏
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn testcase1() {
        assert_eq!(add!(1, 2), 3);
        assert_eq!(add!(1.0, 2.0), 3.0);
    }
}