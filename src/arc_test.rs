use std::sync::Arc;

#[allow(unused)]
fn get_string(re: ???) -> String {
    //请完成这个函数，作用是返回该智能指针中包裹的原字符串
    //返回值类型已给出，请自行写出函数的形参
    todo!();
}

#[allow(unused)]
fn get_strong_count(re: ???) -> usize {
    //请完成这个函数，作用是返回该智能指针中的强引用计数值
    //返回值类型已给出，请自行写出函数的形参
    //注意，请谨慎思考所需的形参类型，使得该Arc智能指针 （不会！！！）因为函数调用而增加引用计数
    todo!();
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