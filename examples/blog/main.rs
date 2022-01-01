mod templates;
mod sample_data;

use std::io::{self, Write};
use std::fs::{self, File};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Post {
    /// The title of the post
    pub title: &'static str,
    /// The URL-friendly identifier for the post
    pub url: &'static str,
    /// The name of the author of the post
    pub author: &'static str,
    /// The raw HTML body of the post
    pub body_html: &'static str,
}

fn main() -> io::Result<()> {
    let output_dir = Path::new("blog-example");
    let posts = sample_data::generate();

    println!("Creating `{}` directory if it doesn't already exist", output_dir.display());
    fs::create_dir_all(output_dir)?;

    let index_page_path = output_dir.join("index.html");
    println!("Writing index page to `{}`", index_page_path.display());
    let mut index_page = File::create(index_page_path)?;
    write!(index_page, "{}", templates::index(posts))?;

    for post in posts {
        let post_page_path = output_dir.join(format!("{}.html", post.url));
        println!("Writing post page to `{}`", post_page_path.display());
        let mut post_page = File::create(post_page_path)?;
        write!(post_page, "{}", templates::post(post))?;
    }

    Ok(())
}
