use std::ops::Add;

#[allow(unused)]
fn add<T: ???>(???) -> ???{
    //实现一个泛型函数，可以实现两个整形或浮点型的数相加
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testcase3() {
        let res1 = add(1, 2);
        let res2 = add(1.0, 2.0);
        assert_eq!(res1, 3);
        assert_eq!(res2, 3.0);
    }
}