// There is a way to avoid this boilerplate by using derive attribute.
// But there is another trait for that, `Debug`, which is for programmer centric output.
// You could implement Debug the same way as `Display`, but there is a #[derive] syntax for that.

#[derive(Debug)]
pub struct Page {
    id: i64,
    title: String,
}

impl Page {
    fn new(id: i64, title: &str) -> Page {
        Page {
            id: id,
            title: String::from(title),
        }
    }
}

fn main() {
    let page = Page::new(12, "Hello");

    // and use {:?} syntax to print Debug output.
    println!("{:?}", page);
}