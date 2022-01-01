use std::borrow::Cow;

use chrono::{Datelike, Local};
use htmlgen::{HtmlElement, HtmlChild, ch, attrs};
use htmlgen::tags::*;

use super::Post;

pub fn index(posts: &[Post]) -> HtmlElement {
    page("Example Blog", ch![
        h1([], ch!["Blog Posts"]),
    ])
}

pub fn post(post: &Post) -> HtmlElement {
    page(format!("{} | Example Blog", post.title), ch![
        //TODO
    ])
}

fn page<S>(title_str: S, body_html: Vec<HtmlChild>) -> HtmlElement
where
    S: Into<Cow<'static, str>>,
{
    html(attrs!{lang="en"}, ch![
        head([], ch![
            meta(attrs!{charset="utf-8"}, []),
            title(title_str),

            link(attrs!{
                rel="stylesheet",
                href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css",
                integrity="sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3",
                crossorigin="anonymous",
            }, []),
        ]),

        body([], ch![
            page_header(),
            page_body(body_html),
            page_footer(),

            script(attrs!{
			  src="https://code.jquery.com/jquery-3.6.0.slim.min.js",
			  integrity="sha256-u7e5khyithlIdTpu22PHhENmPcRdFiHRjhAuHcs05RI=",
			  crossorigin="anonymous",
            }, []),
            script(attrs!{
                src="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/js/bootstrap.bundle.min.js",
                integrity="sha384-ka7Sk0Gln4gmtz2MlQnikT1wXgYsOg+OMhuP+IlRH9sENBO0LRn5q+8nbTov4+1p",
                crossorigin="anonymous",
            }, []),
        ]),
    ])
}

fn page_header() -> HtmlElement {
    div(attrs!{class="container"}, ch![
        header(attrs!{
            class="d-flex flex-wrap justify-content-center py-3 mb-4 border-bottom"
        }, ch![
            a(attrs!{
                class="d-flex align-items-center mb-3 mb-md-0 me-md-auto text-dark text-decoration-none fs-4",
                href="/",
            }, ch![
                "Example Blog",
            ]),
        ])
    ])
}

fn page_body(body_html: Vec<HtmlChild>) -> HtmlElement {
    div(attrs!{class="container"}, ch![
        article([], ch![
            body_html,
        ])
    ])
}

fn page_footer() -> HtmlElement {
    div(attrs!{class="container"}, ch![
        footer(attrs!{class="py-3 my-4 border-top"}, ch![
            p(attrs!{class="text-center text-muted"}, ch![
                format!("Copyright {}. All rights reserved.", Local::now().year()),
            ]),
        ])
    ])
}
