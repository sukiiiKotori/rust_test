// 为下面的结构体添加生命周期
struct Book??? {
    author: &???str,
    title: &???str,
}

mod tests {
    #[test]
    fn testcase4() {
        use crate::lifetimes_test::Book;
        let name = String::from("Jill Smith");
        let title = String::from("Fish Flying");
        let book = Book { author: &name, title: &title };

        println!("{} by {}", book.title, book.author);
    }
}