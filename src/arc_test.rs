use std::sync::Arc;

#[allow(unused)]
fn get_string(re: &String) -> String {
    //请完成这个函数，作用是返回该智能指针中包裹的原字符串
}

#[allow(unused)]
fn get_strong_count(re: &Arc<String>) -> usize {
    //请完成这个函数，作用是返回该智能指针中的强引用计数值
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;
    #[test]
    fn testcase2() {
        let sp_arc = Arc::new("hello world".to_string());
        
        let a = get_string(&sp_arc);
        assert_eq!(a, "hello world".to_string());

        assert_eq!(get_strong_count(&sp_arc), 1);

        let sp2 = Arc::clone(&sp_arc);
        assert_eq!(get_strong_count(&sp_arc), 2);

        let sp3 = Arc::clone(&sp_arc);
        assert_eq!(get_strong_count(&sp_arc), 3);

        drop(sp3);
        assert_eq!(get_strong_count(&sp_arc), 2);

        drop(sp2);
        assert_eq!(get_strong_count(&sp_arc), 1);

    }
}