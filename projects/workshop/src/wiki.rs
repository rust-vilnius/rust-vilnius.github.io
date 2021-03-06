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

    pub fn get_title(&self) -> &str {
        &self.title
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