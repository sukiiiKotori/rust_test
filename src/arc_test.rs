use std::sync::Arc;

fn get_string(re: ???) -> String {
    //请完成这个函数，作用是返回该智能指针中包裹的原字符串
    //返回值类型已给出，请自行写出函数的形参
    todo!();
}

fn get_strong_count(re: ???) -> usize {
    //请完成这个函数，作用是返回该智能指针中的强引用计数值
    //返回值类型已给出，请自行写出函数的形参
    todo!();
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;
    #[test]
    fn testcase2() {
        let sp_arc = Arc::new("hello world".to_string());
        let sp2 = Arc::clone(&sp_arc);
        let a = get_string(&sp_arc);

        assert_eq!(a, "hello world".to_string());
        assert_eq!(get_strong_count(&sp_arc), 2);

    }
}