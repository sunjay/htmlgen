use htmlgen::{HtmlElement, ch, attrs};
use htmlgen::tags::*;

use super::Post;

pub fn index(posts: &[Post]) -> HtmlElement {
    html([], ch![
        head([], ch![
            meta(attrs!{charset: "utf-8"}, []),
        ])
    ])
}

pub fn post(post: &Post) -> HtmlElement {
    todo!()
}
