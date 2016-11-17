---
layout: post
title:  "Boot Camp Rust Workshop"
date:   2016-11-19
categories: meetup workshop
---

Task: count the number of unique words in the "rust" wikipedia page.

We will learn:

- Mutable and immutable variable bindings
- Strings
- References and borrowing
- Mutable borrowing
- Lifetime scope rules
- Functions, borrowing, and universal method syntax
- Ownership, Copy and non-Copy
- Structs and initialization
- Basic trait implementation
- Display and Debug traits
- Modules
- External crates
- Error handling, Result and From trait
- io::Read trait
- Pattern matching and destructuring
- Basic closures
- Iterator usage

* auto-gen TOC:
{:toc}

## println! macro

```rust
/// This is a function, it is named main, that's how rust knows where to start.
fn main() {
    // Macro println is not a simple function, you can see !
    println!("Hello world!");
}
```

## variable bindings

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

## string format

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

## mutable variable bindings

```rust
fn main() {
    // To make mutation possible, add "mut" keyword
    let mut number = 42;
    let mut text = "Number";

    println!("{} is {}", text, number); // OK

    // Then we can assign other values
    number = 43;
    text = "Value";

    println!("{} is {}", text, number); // OK
}
```

## variable type

```rust
fn main() {
    // Type can be explicitly specified after the variable binding
    let number: i64 = 42; // This will use 64 bit integer now
    let text = "Number";

    println!("{} is {}", text, number);
}
```

## finding out the type

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

## 'static memory

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
```

## reference lifetime inference

```rust
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

## String

```rust
fn main() {
    let number: i64 = 42;
    let mut text: String = String::from("Number"); // Enter "String" type

    text.push_str(" Value");

    // Stores string contents on the heap, can be created from string literal
    // How does Rust clean up memory for it? Same way as for "number", at the end of the scope

    println!("{} is {}", text, number);
}
```

## References

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

## Mutable references

```rust
fn main() {
    let number = 42;
    let mut text = String::from("Number");

    // The references can be _uniquely mutable_, that is, while they are in scope
    // the original value can't be read or changed

    let text_ref = &mut text;
    text_ref.push_str(" Value");

    // can not read:
    // cannot borrow `text` as immutable because it is also borrowed as mutable
    println!("{} is {}", text, number);

    // can not change:
    // cannot borrow `text` as mutable more than once at a time
    text.push_str("abc");

    // text_ref active until _here_
}
```

## reference lifetime

```rust
fn main() {
    let number = 42;
    let mut text = String::from("Number");

    // The scope of mutable borrow can be reduced by creating a block

    {
        let text_ref = &mut text;
        text_ref.push_str(" Value");
    }

    println!("{} is {}", text, number); // OK

    text.push_str("abc"); // OK

    println!("{} is {}", text, number); // OK
}
```

## functions that borrow

```rust
// let's move that scope into a function
fn append_to_string(text: &mut String) {
    text.push_str(" Value");
}

fn main() {
    let number = 42;
    let mut text = String::from("Number");

    // borrow that lasts only as long as function this way
    append_to_string(&mut text);

    println!("{} is {}", text, number); // OK
}
```

## multiple references in function arguments

```rust
// let's make a function where we pass the appended string over the argument
fn append_to_string(text: &mut String, what: &str) {
    text.push_str(what);
}

fn main() {
    let number = 42;
    let mut text = String::from("Number");

    // pass the argument here
    append_to_string(&mut text, " Value");

    // it works!

    println!("{} is {}", text, number); // OK
}
```

## let's try to pass reference to the same text as second argument

```rust
// let's make a function where we pass the appended string over the argument
fn append_to_string(text: &mut String, what: &str) {
    text.push_str(what);
}

fn main() {
    let number = 42;
    let mut text = String::from("Number");

    // now we get the infamous error
    // cannot borrow `text` as immutable because it is also borrowed as mutable
    append_to_string(&mut text, &text);
    //                    ----   ^^^^- mutable borrow ends here
    //                    |      |
    //                    |      immutable borrow occurs here
    //                    mutable borrow occurs here

    // question: why this would be bad?

    println!("{} is {}", text, number); // OK
}
```

## universal function call syntax

```rust
fn main() {
    let number = 42;
    let mut text = String::from("Number");

    // it's the same error we would get when doing that directly on text
    text.push_str(&text);
//  ----           ^^^^- mutable borrow ends here
//  |              |
//  |              immutable borrow occurs here
//  mutable borrow occurs here

    // if you look at
    // fn push_str(&mut self, string: &str);
    // signature, it takes mutably borrowed "self"
    // as first argument

    // it is the same as calling
    String::push_str(&mut text, &text);
    //                    ----   ^^^^- mutable borrow ends here
    //                    |      |
    //                    |      immutable borrow occurs here
    //                    mutable borrow occurs here

    // rust borrows automatically when calling methods that
    // use borrowed &self or &mut self

    println!("{} is {}", text, number); // OK
}
```

## variable assignments

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

## error using moved value

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

## difference between Copy and non-Copy

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

## method .clone

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

## struct

```rust
struct Page {
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

## constructor and returning values from functions

```rust
struct Page {
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

## implementing Display trait

```rust
// if we try to `println!("Page: {}", page);` we will find it does not implement
// `Display` trait.

// So, what are traits and how do we implement them?

// This particular trait lives in fmt module that we can use.
use std::fmt;

struct Page {
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

// If we open `Display` documentation, we will find example
// implementation for Point which we can modify to work for `Page`
impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.title)
    }
}

fn main() {
    let page = Page::new(12, "Hello");

    // and now displaying page works!
    println!("Page: {}", page);
}
```

## the Debug trait and derive

```rust
// There is a way to avoid this boilerplate by using derive attribute.
// But there is another trait for that, `Debug`, which is for programmer centric output.
// You could implement Debug the same way as `Display`, but there is a #[derive] syntax for that.

#[derive(Debug)]
struct Page {
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
```

## modules

```rust
// Wrap page in mod {}
mod wiki {
    // make `fn new` pub.
    // make `Page` pub.

    #[derive(Debug)]
    pub struct Page {
        id: i64,
        title: String,
    }

    impl Page {
        pub fn new(id: i64, title: &str) -> Page {
            Page {
                id: id,
                title: String::from(title),
            }
        }
    }
}

fn main() {
    let page = wiki::Page::new(12, "Hello");

    println!("{:?}", page);
}
```

## public method that returns the result

```rust
mod wiki {
    #[derive(Debug)]
    pub struct Page {
        id: i64,
        title: String,
    }

    impl Page {
        pub fn new(id: i64, title: &str) -> Page {
            Page {
                id: id,
                title: String::from(title),
            }
        }
    }

    // Create a function to search for a page
    // function can fail - so we return Result that can be either Ok or Err

    pub fn search(query: &str) -> Result<Page, Error> {
        if query.is_empty() {
            return Err(Error::QueryIsEmpty);
        }

        // return dummy page for now
        Ok(Page::new(0, "Fake page"))
    }

    // For Error, we create our own "wiki" Error
    // so far it has only one case - when search query is empty

    #[derive(Debug)]
    pub enum Error {
        QueryIsEmpty,
    }
}

fn main() {
    let page = wiki::search("").expect("failed to find page");

    println!("{:?}", page);
}
```

## perform "get" request

```rust
// for http requests, we will use new "reqwest" crate
// add `reqwest = "0.1"` to [dependencies] section in Cargo.toml

// then add reference to this crate
extern crate reqwest;

mod wiki {
    // then use reqwest in inner module.
    use reqwest;
    // we will need Read trait to call method `read_to_string` on Response.
    use std::io::Read;

    #[derive(Debug)]
    pub struct Page {
        id: i64,
        title: String,
    }

    impl Page {
        pub fn new(id: i64, title: &str) -> Page {
            Page {
                id: id,
                title: String::from(title),
            }
        }
    }

    pub fn search(query: &str) -> Result<Page, Error> {
        if query.is_empty() {
            return Err(Error::QueryIsEmpty);
        }

        // locate reqwest crate in crates io: https://crates.io/crates/reqwest
        // find the documentation
        let mut resp = reqwest::get("https://en.wikipedia.org").unwrap(); // THIS CAN FAIL
        let mut contents = String::new();

        // `read_to_string` requires both resp and contents to be mutably borrowed
        resp.read_to_string(&mut contents).unwrap(); // THIS CAN FAIL

        // return contents in title
        Ok(Page::new(0, &contents))
    }

    #[derive(Debug)]
    pub enum Error {
        QueryIsEmpty,
    }
}

fn main() {
    let page = wiki::search("rust").expect("failed to find page");

    println!("{:?}", page);
}
```

## handle all possible errors

```rust
extern crate reqwest;

mod wiki {
    use reqwest;
    use std::io::Read;
    use std::io; // added std::io to reference io::Error easier
    use std::result; // added to reference result::Result easier

    #[derive(Debug)]
    pub struct Page {
        id: i64,
        title: String,
    }

    impl Page {
        pub fn new(id: i64, title: &str) -> Page {
            Page {
                id: id,
                title: String::from(title),
            }
        }
    }

    // to handle the errors, we need to convert them from external Error to our Error
    // then we can use try!() or new question_mark syntax to short-circuit function on error
    // usual practice is to add enum case for every possible external error to the wrapper error

    pub fn search(query: &str) -> Result<Page> {
        if query.is_empty() {
            return Err(Error::QueryIsEmpty);
        }

        let mut resp = reqwest::get("https://en.wikipedia.org")?; // use question mark
        let mut contents = String::new();
        resp.read_to_string(&mut contents)?; // use question mark

        Ok(Page::new(0, &contents))
    }

    #[derive(Debug)]
    pub enum Error {
        QueryIsEmpty,
        Reqwest(reqwest::Error),
        Io(io::Error),
    }

    // implement conversion from external errors to our wrapper error

    impl From<reqwest::Error> for Error {
        fn from(other: reqwest::Error) -> Error {
            Error::Reqwest(other)
        }
    }

    impl From<io::Error> for Error {
        fn from(other: io::Error) -> Error {
            Error::Io(other)
        }
    }

    pub type Result<T> = result::Result<T, Error>;
}

fn main() {
    let page = wiki::search("rust").expect("failed to find page");

    println!("{:?}", page);
}
```

## add a function to create url

```rust
extern crate reqwest;

mod wiki {
    use reqwest;
    use reqwest::Url;
    use std::io::Read;
    use std::io;
    use std::result;

    #[derive(Debug)]
    pub struct Page {
        id: i64,
        title: String,
    }

    impl Page {
        pub fn new(id: i64, title: &str) -> Page {
            Page {
                id: id,
                title: String::from(title),
            }
        }
    }

    // wikipedia API uses this kind of url to search:
    // https://en.wikipedia.org/w/api.php?action=query&titles=Main%20Page&format=json&formatversion=2
    // response looks like this: {"batchcomplete":true,"query":{"pages":[{"pageid":15580374,"ns":0,"title":"Main Page"}]}}

    // let's create function to construct url for search

    fn get_search_url(query: &str) -> Result<Url> {
        let mut url = Url::parse("https://en.wikipedia.org/w/api.php")?;
        url.query_pairs_mut()
            .append_pair("action", "query")
            .append_pair("format", "json")
            .append_pair("prop", "extracts")
            .append_pair("formatversion", "2")
            .append_pair("titles", query);

        Ok(url)
    }

    pub fn search(query: &str) -> Result<Page> {
        if query.is_empty() {
            return Err(Error::QueryIsEmpty);
        }

        let mut resp = reqwest::get(get_search_url(query)?)?;
        let mut contents = String::new();
        resp.read_to_string(&mut contents)?;

        Ok(Page::new(0, &contents))
    }

    #[derive(Debug)]
    pub enum Error {
        QueryIsEmpty,
        Reqwest(reqwest::Error),
        Url(reqwest::UrlError), // we will also need another error case for UrlError
        Io(io::Error),
    }

    // implement conversion from external errors to our wrapper error

    impl From<reqwest::Error> for Error {
        fn from(other: reqwest::Error) -> Error {
            Error::Reqwest(other)
        }
    }

    // and conversion from UrlError
    impl From<reqwest::UrlError> for Error {
        fn from(other: reqwest::UrlError) -> Error {
            Error::Url(other)
        }
    }

    impl From<io::Error> for Error {
        fn from(other: io::Error) -> Error {
            Error::Io(other)
        }
    }

    pub type Result<T> = result::Result<T, Error>;
}

fn main() {
    let page = wiki::search("rust").expect("failed to find page");

    println!("{:?}", page);
}
```

## extract content download to another function

```rust
extern crate reqwest;

mod wiki {
    use reqwest;
    use reqwest::Url;
    use std::io::Read;
    use std::io;
    use std::result;

    #[derive(Debug)]
    pub struct Page {
        id: i64,
        title: String,
    }

    impl Page {
        pub fn new(id: i64, title: &str) -> Page {
            Page {
                id: id,
                title: String::from(title),
            }
        }
    }

    fn get_search_url(query: &str) -> Result<Url> {
        let mut url = Url::parse("https://en.wikipedia.org/w/api.php")?;
        url.query_pairs_mut()
            .append_pair("action", "query")
            .append_pair("format", "json")
            .append_pair("prop", "extracts")
            .append_pair("formatversion", "2")
            .append_pair("titles", query);

        Ok(url)
    }

    // move url contents downloading to separate function

    fn get_contents_from_url(url: Url) -> Result<String> {
        let mut resp = reqwest::get(url)?;
        let mut contents = String::new();
        resp.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn search(query: &str) -> Result<Page> {
        if query.is_empty() {
            return Err(Error::QueryIsEmpty);
        }

        let contents = get_contents_from_url(
            get_search_url(query)?
        )?;

        Ok(Page::new(0, &contents))
    }

    #[derive(Debug)]
    pub enum Error {
        QueryIsEmpty,
        Reqwest(reqwest::Error),
        Url(reqwest::UrlError),
        Io(io::Error),
    }

    impl From<reqwest::Error> for Error {
        fn from(other: reqwest::Error) -> Error {
            Error::Reqwest(other)
        }
    }

    // and conversion from UrlError
    impl From<reqwest::UrlError> for Error {
        fn from(other: reqwest::UrlError) -> Error {
            Error::Url(other)
        }
    }

    impl From<io::Error> for Error {
        fn from(other: io::Error) -> Error {
            Error::Io(other)
        }
    }

    pub type Result<T> = result::Result<T, Error>;
}

fn main() {
    let page = wiki::search("rust").expect("failed to find page");

    println!("{:?}", page);
}
```

## include json deserialization library and try deserializing response contents

```rust
extern crate reqwest;

// add
// serde = "0.8"
// serde_json = "0.8"
// in Cargo.toml

extern crate serde_json; // reference external crate serde_json

mod wiki {
    use reqwest;
    use reqwest::Url;

    // use serde_json and serde_json::Value
    use serde_json;
    use serde_json::Value;

    use std::io::Read;
    use std::io;
    use std::result;

    #[derive(Debug)]
    pub struct Page {
        id: i64,
        title: String,
    }

    impl Page {
        pub fn new(id: i64, title: &str) -> Page {
            Page {
                id: id,
                title: String::from(title),
            }
        }
    }

    fn get_search_url(query: &str) -> Result<Url> {
        let mut url = Url::parse("https://en.wikipedia.org/w/api.php")?;
        url.query_pairs_mut()
            .append_pair("action", "query")
            .append_pair("format", "json")
            .append_pair("prop", "extracts")
            .append_pair("formatversion", "2")
            .append_pair("titles", query);

        Ok(url)
    }

    // move url contents downloading to separate function

    fn get_contents_from_url(url: Url) -> Result<String> {
        let mut resp = reqwest::get(url)?;
        let mut contents = String::new();
        resp.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn search(query: &str) -> Result<Page> {
        if query.is_empty() {
            return Err(Error::QueryIsEmpty);
        }

        let contents = get_contents_from_url(
            get_search_url(query)?
        )?;

        // deserialize contents to value
        let value: Value = serde_json::from_str(&contents)?;

        // print value for now
        println!("{:?}", value);

        Ok(Page::new(0, &contents))
    }

    #[derive(Debug)]
    pub enum Error {
        QueryIsEmpty,
        Reqwest(reqwest::Error),
        Url(reqwest::UrlError),
        Io(io::Error),
        Serde(serde_json::Error),
    }

    impl From<reqwest::Error> for Error {
        fn from(other: reqwest::Error) -> Error {
            Error::Reqwest(other)
        }
    }

    impl From<reqwest::UrlError> for Error {
        fn from(other: reqwest::UrlError) -> Error {
            Error::Url(other)
        }
    }

    impl From<io::Error> for Error {
        fn from(other: io::Error) -> Error {
            Error::Io(other)
        }
    }

    // add from serde_json::Error
    impl From<serde_json::Error> for Error {
        fn from(other: serde_json::Error) -> Error {
            Error::Serde(other)
        }
    }

    pub type Result<T> = result::Result<T, Error>;
}

fn main() {
    let page = wiki::search("rust").expect("failed to find page");

    println!("{:?}", page);
}
```

## add method to extract pages from json response

```rust
// some content is skipped, see previous example for full code

#[derive(Debug)]
pub struct Page {
    id: i64,
    title: String,
    contents: String, // add a contents field
}

impl Page {
    pub fn new(id: i64, title: &str, contents: &str) -> Page {
        Page {
            id: id,
            title: String::from(title),
            contents: String::from(contents), // initialize it
        }
    }
}

// new method to extract pages from json value

fn get_pages_from_json_value(value: Value) -> Vec<Page> {
    let mut results = Vec::new();

    if let Some(array) = value.pointer("/query/pages").and_then(|p| p.as_array()) {
        for item in array {
            let id = item.pointer("/pageid").and_then(|p| p.as_i64());
            let title = item.pointer("/title").and_then(|p| p.as_str());
            let contents = item.pointer("/extract").and_then(|p| p.as_str());

            match (id, title, contents) {
                (Some(id), Some(title), Some(contents)) =>
                    results.push(Page::new(id, title, contents)),
                _ => continue,
            }
        }
    }

    results
}

pub fn search(query: &str) -> Result<Page> {
    if query.is_empty() {
        return Err(Error::QueryIsEmpty);
    }

    let contents = get_contents_from_url(
        get_search_url(query)?
    )?;

    let pages = get_pages_from_json_value(
        serde_json::from_str(&contents)?
    );

    // convert pages into iterator that owns them, then take the next value or return
    // the error
    pages.into_iter().next().ok_or(Error::NotFound)
}

#[derive(Debug)]
pub enum Error {
    QueryIsEmpty,
    NotFound, // add NotFound error
    Reqwest(reqwest::Error),
    Url(reqwest::UrlError),
    Io(io::Error),
    Serde(serde_json::Error),
}
```

## add method on Page to return str reference to contents

`main.rs`

```rust
pub mod wiki; // move everything inside into wiki.rs file

fn main() {
    let page = wiki::search("rust").expect("failed to find page");

    // use get_contents method to print only the contents
    println!("Page contents: {:?}", page.get_contents());
}
```

`wiki.rs`

```rust
impl Page {
    pub fn new(id: i64, title: &str, contents: &str) -> Page {
        Page {
            id: id,
            title: String::from(title),
            contents: String::from(contents),
        }
    }

    // add method
    // question: why doesn't Rust complain here?
    pub fn get_contents(&self) -> &str {
        &self.contents
    }
}
```

## stripping html tags with some amonia

`main.rs`

```rust
extern crate reqwest;
extern crate serde_json;

// In Cargo.toml, add
// ammonia = "0.1"

// add reference to ammonia
extern crate ammonia;

use ammonia::Ammonia;
use std::collections::HashSet;

pub mod wiki;

fn main() {
    let page = wiki::search("rust").expect("failed to find page");
    let html = page.get_contents();

    // ammonia::clean() would initialize Ammonia with allowed tag list, but we don't need any tags

    // But we can pull default Ammonia initialization and override only the "tags" field with
    // empty tag set
    let defaults = Ammonia::default();
    let ammonia = ammonia::Ammonia { tags: HashSet::new(), ..defaults };

    // this way we strip all the tags
    let clean_text = ammonia.clean(&html);

    println!("Page contents: {:?}", clean_text);
}
```

## collecting iterator into HashSet

`main.rs`

```rust
extern crate reqwest;
extern crate serde_json;
extern crate ammonia;

use ammonia::Ammonia;
use std::collections::HashSet;

pub mod wiki;

fn main() {
    let page = wiki::search("rust").expect("failed to find page");
    let html = page.get_contents();

    let defaults = Ammonia::default();
    let ammonia = ammonia::Ammonia { tags: HashSet::new(), ..defaults };

    let clean_text = ammonia.clean(&html);

    // split by whitespace and collect into hash set of string references
    let unique_words: HashSet<&str> = clean_text
        .split_whitespace()
        .collect();

    println!("Unique word count: {:?}", unique_words.len());
}
```

`wiki.rs`

```rust
use reqwest;
use reqwest::Url;
use serde_json;
use serde_json::Value;
use std::io::Read;
use std::io;
use std::result;

#[derive(Debug)]
pub struct Page {
    id: i64,
    title: String,
    contents: String,
}

impl Page {
    pub fn new(id: i64, title: &str, contents: &str) -> Page {
        Page {
            id: id,
            title: String::from(title),
            contents: String::from(contents),
        }
    }

    pub fn get_contents(&self) -> &str {
        &self.contents
    }
}

fn get_search_url(query: &str) -> Result<Url> {
    let mut url = Url::parse("https://en.wikipedia.org/w/api.php")?;
    url.query_pairs_mut()
        .append_pair("action", "query")
        .append_pair("format", "json")
        .append_pair("prop", "extracts")
        .append_pair("formatversion", "2")
        .append_pair("titles", query);

    Ok(url)
}

fn get_contents_from_url(url: Url) -> Result<String> {
    let mut resp = reqwest::get(url)?;
    let mut contents = String::new();
    resp.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_pages_from_json_value(value: Value) -> Vec<Page> {
    let mut results = Vec::new();

    if let Some(array) = value.pointer("/query/pages").and_then(|p| p.as_array()) {
        for item in array {
            let id = item.pointer("/pageid").and_then(|p| p.as_i64());
            let title = item.pointer("/title").and_then(|p| p.as_str());
            let contents = item.pointer("/extract").and_then(|p| p.as_str());

            match (id, title, contents) {
                (Some(id), Some(title), Some(contents)) =>
                    results.push(Page::new(id, title, contents)),
                _ => continue,
            }
        }
    }

    results
}

pub fn search(query: &str) -> Result<Page> {
    if query.is_empty() {
        return Err(Error::QueryIsEmpty);
    }

    let contents = get_contents_from_url(
        get_search_url(query)?
    )?;

    let pages = get_pages_from_json_value(
        serde_json::from_str(&contents)?
    );

    pages.into_iter().next().ok_or(Error::NotFound)
}

#[derive(Debug)]
pub enum Error {
    QueryIsEmpty,
    NotFound, // add NotFound error
    Reqwest(reqwest::Error),
    Url(reqwest::UrlError),
    Io(io::Error),
    Serde(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(other: reqwest::Error) -> Error {
        Error::Reqwest(other)
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(other: reqwest::UrlError) -> Error {
        Error::Url(other)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Error {
        Error::Io(other)
    }
}

impl From<serde_json::Error> for Error {
    fn from(other: serde_json::Error) -> Error {
        Error::Serde(other)
    }
}

pub type Result<T> = result::Result<T, Error>;
```
