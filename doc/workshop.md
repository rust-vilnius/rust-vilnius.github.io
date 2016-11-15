* auto-gen TOC:
{:toc}

# println! macro

```rust
/// This is a function, it is named main, that's how rust knows where to start.
fn main() {
    // Macro println is not a simple function, you can see !
    println!("Hello world!");
}
```

# variable bindings

```rust
fn main() {
    // Let's create a some variable _bindings_
    let number = 42;
    let text = "Number";

    // However, can't use text directly in macro, because macro treats
    // first argument string literal differently

    println!(text); // error: expected a literal
}
```

# string format

```rust
fn main() {
    let number = 42;
    let text = "Number";
    
    // Why? The println! can also format strings, and checks if
    // the number of placeholders match the number or passed arguments
    println!("{} is {}", text, number); // OK
    println!("{1} is {0}", number, text); // Same as above
    println!("{a} is {b}", a = text, b = number); // Same as above

    // The macro is more than a simple function, some magic behind
}
```

# variable type

```rust
fn main() {
    // Type can be explicitly specified after the variable binding
    let number: i64 = 42; // This will use 64 bit integer now
    let text = "Number";

    println!("{} is {}", text, number);
}
```

# finding out the type

```rust
fn main() {
    let number: i64 = 42;
    // but what's the type of "text"? String?
    // the common trick to find a type of anything - specify different type and let the compiler
    // tell you
    // there is a special type called "unit" type, it looks like "()"
    // you can also think of it as empty tuple
    let text: () = "Number";

    // Error:
    // = note: expected type `()`
    // = note:    found type `&'static str`

    // What is "&'static str"?

    println!("{} is {}", text, number);
}
```

# 'static memory

```rust
fn main() {
    let number: i64 = 42;
    let text: &'static str = "Number";

    // &str is a string _slice_
    // you can think of it as a reference to a memory location with a length
    // or as a partial view of string

    // references are safe pointers

    // in Rust, references can have _names_
    // names refer to a scope the reference comes from
    // the 'static refers to the compiled executable's memory

    println!("{} is {}", text, number);
}

/// # reference lifetime inference

fn main() {
    let number: i64 = 42;
    let text: &str = "Number"; // We can skip 'static from reference name, Rust can infer that

    // The problem with references is that they can't be mutated
    // they are like "const T*" in C++, but they actually can't be mutated (outside the "unsafe" block)

    // Why? Remember where it is stored - in the compiled binary.
    // To mutate something, you need to store in location where mutation is possible.

    println!("{} is {}", text, number);
}
```

# String

```rust
fn main() {
    let number: i64 = 42;
    let text: String = String::from("Number"); // Enter "String" type

    // Stores string contents on the heap, can be created from string literal
    // How does Rust clean up memory for it? Same way as for "number", at the end of the scope

    println!("{} is {}", text, number);
}
```

# Ownership

```rust
fn main() {
    let number: i64 = 42;
    let text: String = String::from("Number");

    // Quick ownership lesson
    // What if... we create a new variables, number2 and text2

    let number2 = number;
    let text2 = text;

    println!("{} is {}", text2, number2);

    // It works! And does not leak memory. How?
}
```

# Ownership II

```rust
fn main() {
    let number: i64 = 42;
    let text: String = String::from("Number");

    // You would think String was _copied_, but Rust chooses unconventional solution here

    let number2 = number;
    let text2 = text;

    // If we try to print original "number" and "text", "number" will work, but "text" will fail

    println!("number: {}", number);
    println!("text: {}", text);

    //    |
    // 8  |     let text2 = text;
    //    |         ----- value moved here
    // ...
    // 13 |     println!("number: {}", text);
    //    |                            ^^^^ value used here after move
    //    |
    //    = note: move occurs because `text` has type `std::string::String`, which does not implement the `Copy` trait

    println!("{} is {}", text2, number2);
}
```

# Ownership III

```rust
fn main() {
    let number: i64 = 42;
    let text: String = String::from("Number");

    // Rust _moves_ any non-primitive type by default

    let number2 = number; // primitive i64, Copy
    let text2 = text; // non-primitive String, move

    println!("number: {}", number);
    println!("text: {}", text); // value used here after move to text2

    println!("{} is {}", text2, number2);
}
```

# Ownership IV

```rust
fn main() {
    let number: i64 = 42;
    let text: String = String::from("Number");

    // There is easy work-around, however - explicit call to .clone() that does deep copy

    let number2 = number; // primitive i64, Copy
    let text2 = text.clone(); // cloned, text can still be used

    println!("number: {}", number);
    println!("text: {}", text); // OK

    println!("{} is {}", text2, number2); // OK

    // big question here is - why?

    // but this how Rust is efficient by default
    // String contains a pointer to a single memory location,
    // that is created and destroyed exactly once
}
```

# Borrowing

```rust
fn main() {
    let number = 42;
    let text = String::from("Number");

    let text_ref = &text; // create &String reference to String _container_
    println!("{} is {}", text_ref, number); // OK

    // Prints "Number is 42"

    let text_ref = &text[0..3]; // create &str reference to string contents from [0 to 3)
    println!("{} is {}", text_ref, number); // OK

    // Prints "Num is 42"

    let text_ref = &text[..]; // create &str reference to the whole String
    println!("{} is {}", text_ref, number); // OK

    // Prints "Number is 42"

    // We can have as many immutable references as we like
}
```

# Mutable bindings

```rust
fn main() {
    let number = 42;
    let mut text = String::from("Number");

    // We use String and pass it around so that we can mutate it

    text.push_str(" really");

    println!("{} is {}", text, number);
}
```

# Mutable references

```rust
fn main() {
    let number = 42;
    let mut text = String::from("Number");

    // The references can be _uniquely mutable_, that is, while they are in scope
    // the original value can't be read or changed

    let text_ref = &mut text;
    text_ref.push_str(" really");

    // can not read:
    // cannot borrow `text` as immutable because it is also borrowed as mutable
    println!("{} is {}", text, number);

    // can not change:
    // cannot borrow `text` as mutable more than once at a time
    text.push_str("abc");

    // text_ref active until _here_
}
```

# Scoped lifetime

```rust
fn main() {
    let number = 42;
    let mut text = String::from("Number");

    // The scope of mutable borrow can be reduced by creating a block

    {
        let text_ref = &mut text;
        text_ref.push_str(" really");
    }

    println!("{} is {}", text, number); // OK

    text.push_str("abc"); // OK

    println!("{} is {}", text, number); // OK
}
```

# struct

```rust
pub struct Page {
    id: i64,
    title: String,
}

fn main() {
    // Structure initialization

    let page = Page {
        id: 12,
        title: String::from("Hello")
    };

    println!("Page id: {}, title: {}", page.id, page.title);
}
```

# constructor and returning values from functions

```rust
pub struct Page {
    id: i64,
    title: String,
}

impl Page { // implementation in separate block
    fn new(id: i64, title: &str) -> Page { // convenience constructor, or "static method"
        Page {
            id: id,
            title: String::from(title),
        } // no "return" statement necessary, but do not place ;
    }
}

fn main() {
    // Initialization is now cleaner
    let page = Page::new(12, "Hello");

    println!("Page id: {}, title: {}", page.id, page.title);
}
```

# static lifetime

```rust
pub struct Page {
    id: i64,
    title: String,
}

impl Page {
    // there is only one specific lifetime name, and it's 'static
    // we know that "Hello" is in 'static memory, so we can make this method disallow
    // anything from shorter scope
    fn new(id: i64, title: &'static str) -> Page {
        Page {
            id: id,
            title: String::from(title),
        }
    }
}

fn main() {
    // this still works
    let page = Page::new(12, "Hello");

    // however a reference to memory in shorter scope does not
    let title = String::from("Bye");

    // take a reference to string contents (effectively a pointer)
    let page = Page::new(12, &title[..]); // error: `title` does not live long enough
    //                        ^^^^^ does not live long enough

    println!("Page id: {}, title: {}", page.id, page.title);
}
```

# generic lifetime

```rust
pub struct Page {
    id: i64,
    title: String,
}

impl Page {
    // the default "behind the scenes" behavior is to make reference generic over the scope
    // whe reference points to
    fn new<'a>(id: i64, title: &'a str) -> Page {
        // this is elided and there is usually no need to do this manually
        // but is much easier to introduce here :)
        Page {
            id: id,
            title: String::from(title),
        }
    }
}

fn main() {
    let page = Page::new(12, "Hello");

    println!("Page id: {}, title: {}", page.id, page.title);
}
```

# generic lifetime

```rust
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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<'a> From<(i64, &'a str)> for Page {
    fn from((id, title): (i64, &str)) -> Self {
        Page::new(id, title)
    }
}

fn main() {
    let page = Page::new(12, "Hello");

    println!("Page: {}", page);
}
```

# Final

```rust
extern crate serde_json;
extern crate hyper;

mod wikipedia {
    use serde_json;
    use hyper::client::Client;
    use hyper::Url;
    use std::io::Read;

    #[derive(Debug)]
    pub struct Page {
        pageid: i64,
        title: String,
    }

    pub fn search_for_page(query: &str) -> Vec<Page> {
        let value = serde_json::from_str(&get_search_response_string(query))
            .expect("failed to decode json string");

//        value.pointer("/query/pages")
//            .and_then(|p| p.as_array())
//            .map(|array| array.into_iter()
//                .filter_map(|item| {
//                    let id = item.pointer("/pageid").and_then(|p| p.as_i64());
//                    let title = item.pointer("/title").and_then(|p| p.as_str());
//
//                    match (id, title) {
//                        (Some(id), Some(title)) => Some((id, title)),
//                        _ => None,
//                    }
//                })
//                .map(|(id, title)| Page {
//                    pageid: id,
//                    title: title.to_string(),
//                })
//                .collect())
//            .unwrap_or(Vec::new())

        if let Some(array) = value.pointer("/query/pages").and_then(|p| p.as_array()) {

            let mut results = Vec::new();

            for item in array {
                let id = item.pointer("/pageid").and_then(|p| p.as_i64());
                let title = item.pointer("/title").and_then(|p| p.as_str());

                match (id, title) {
                    (Some(id), Some(title)) => results.push(Page {
                        pageid: id,
                        title: title.to_string(),
                    }),
                    _ => continue,
                }
            }

            return results;
        }

        Vec::new()
    }

    fn get_search_response_string(query: &str) -> String {
        // https://crates.io/crates/hyper
        // http://hyper.rs/hyper/v0.9.12/hyper/client/index.html

        let client = Client::new();
        let mut response = client.get(get_search_url(query))
            .send()
            .expect("failed!");

        println!("{:#?}", response);

        let mut result = String::new();

        response.read_to_string(&mut result)
            .expect("failed to read");

        result
    }

    fn get_search_url(query: &str) -> Url {
        let mut url = Url::parse("https://en.wikipedia.org/w/api.php")
            .expect("failed to parse url");

        url.query_pairs_mut()
            .append_pair("action", "query")
            .append_pair("format", "json")
            .append_pair("formatversion", "2")
            .append_pair("titles", query);

        url
    }
}

fn main() {
    let contents = wikipedia::search_for_page("rust");
    println!("{:#?}", contents);
}
```