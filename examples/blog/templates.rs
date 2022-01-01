use std::borrow::Cow;

use chrono::{Datelike, Local};
use htmlgen::{HtmlElement, HtmlChild, ch, attrs};
use htmlgen::tags::*;

use super::Post;

pub fn index(posts: &[Post]) -> HtmlElement {
    page("Example Blog", ch![
        h1([], ch!["Blog Posts"]),

        div(attrs!{class="list-group"}, ch![
            p([], ch!["The latest breaking news and opinions from all around the web:"]),
            posts_list(posts),
        ])
    ])
}

fn posts_list(posts: &[Post]) -> Vec<HtmlChild> {
    posts.iter().rev().map(|post| {
        a(attrs!{
            href=format!("{}.html", post.url),
            class="list-group-item list-group-item-action d-flex gap-3 py-3",
        }, ch![
            div(attrs!{class="d-flex gap-2 w-100 justify-content-between"}, ch![
                div([], ch![
                    h6(attrs!{class="mb-0"}, ch![post.title.to_string()]),
                    p(attrs!{class="mb-0 opacity-75"}, ch![
                        post.author.to_string(),
                    ]),
                ]),
                small(attrs!{class="opacity-50 text-nowrap"}, ch![
                    post.published.format("%F").to_string(),
                ]),
            ]),
        ])
    }).map(HtmlChild::from).collect()
}

pub fn post(post: &Post) -> HtmlElement {
    page(format!("{} | Example Blog", post.title), ch![
        article([], ch![
            h2(attrs!{class="fs-2"}, ch![post.title.to_string()]),
            div(attrs!{class="d-flex gap-2 w-100 justify-content-between"}, ch![
                h6(attrs!{class="opacity-75 mb-0"}, ch![
                    em([], ch![post.author.to_string()]),
                ]),
                small(attrs!{class="opacity-50 text-nowrap"}, ch![
                    post.published.format("%b %d, %Y").to_string(),
                ]),
            ]),
            p([], []),
            raw_html(post.body_html.to_string()),
        ]),
    ])
}

fn page<S>(title_str: S, content: Vec<HtmlChild>) -> HtmlElement
where
    S: Into<Cow<'static, str>>,
{
    html(attrs!{lang="en", class="h-100"}, ch![
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

        body(attrs!{class="d-flex flex-column h-100"}, ch![
            page_header(),
            page_body(content),
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

fn page_body(content: Vec<HtmlChild>) -> HtmlElement {
    div(attrs!{class="container"}, ch![
        main([], ch![
            content,
            // Ensures there is some space after the content
            p([], []),
        ])
    ])
}

fn page_footer() -> HtmlElement {
    footer(attrs!{class="footer mt-auto py-3 bg-light border-top"}, ch![
        div(attrs!{class="container"}, ch![
            span(attrs!{class="text-center text-muted"}, ch![
                "Copyright ",
                Local::now().year().to_string(),
                ". All rights reserved.",
            ]),
        ])
    ])
}
