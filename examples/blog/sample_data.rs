use super::Post;

use rand::prelude::*;
use chrono::{Local, Duration, TimeZone};
use lipsum::lipsum_words_from_seed;
use slugmin::slugify_normal;
use inflector::cases::titlecase::to_title_case;

/// Generates some sample posts
pub fn generate() -> Vec<Post> {
    let mut posts = Vec::new();

    let mut rng = StdRng::seed_from_u64(10203049);

    let mut published = Local.ymd(2010, 01, 01);
    for _ in 0..15 {
        published = published + Duration::days(rng.gen_range(7..=60));

        let title = to_title_case(&lipsum_words_from_seed(rng.gen_range(2..=8), rng.gen()));
        posts.push(Post {
            title: title.clone(),
            url: slugify_normal(title, false).replace(' ', "-"),
            author: to_title_case(&lipsum_words_from_seed(2, rng.gen())),
            body_html: generate_article(&mut rng),
            published,
        })
    }

    posts
}

fn generate_article(rng: &mut StdRng) -> String {
    let mut article = String::new();

    for _ in 0..rng.gen_range(3..=56) {
        article.push_str(&generate_paragraph(rng));
    }

    article
}

fn generate_paragraph(rng: &mut StdRng) -> String {
    let mut paragraph = String::new();

    paragraph.push_str("<p>");

    // Generate random words
    let base_text = lipsum_words_from_seed(rng.gen_range(10..=78), rng.gen());
    for word in base_text.split_whitespace() {
        // Apply random formatting
        match rng.gen_range(1..=100) {
            1..=2 => {
                paragraph.push_str("<strong>");
                paragraph.push_str(word);
                paragraph.push_str("</strong>");
            },

            3..=5 => {
                paragraph.push_str("<em>");
                paragraph.push_str(word);
                paragraph.push_str("</em>");
            },

            _ => {
                paragraph.push_str(word);
            },
        }
        paragraph.push_str(" ");
    }

    paragraph.truncate(paragraph.trim_end().len());
    paragraph.push_str("</p>");

    paragraph
}
