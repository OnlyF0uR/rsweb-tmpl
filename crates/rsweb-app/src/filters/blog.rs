use pulldown_cmark::{html, Parser};
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug)]
pub struct Blog {
    pub slug: String,
    pub metadata: BlogMetadata,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlogMetadata {
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub date: String,
    pub tags: Vec<String>,
    pub hero_img: Option<String>,
}

fn parse_front_matter(md_content: &str) -> (Option<BlogMetadata>, String) {
    // Find the front matter block and separate it from the content
    if let Some(pos) = md_content.find("---") {
        let end_pos = md_content[pos + 3..]
            .find("---")
            .map(|e| pos + 3 + e)
            .unwrap_or_else(|| md_content.len());
        let front_matter = &md_content[pos + 3..end_pos];

        // Parse front matter as YAML
        if let Ok(metadata) = serde_yaml_ng::from_str::<BlogMetadata>(front_matter) {
            // Get the content after front matter
            let content = md_content[end_pos + 3..].trim_start();
            return (Some(metadata), content.to_string());
        }
    }
    (None, md_content.to_string())
}

fn markdown_to_html(md_content: &str) -> String {
    let parser = Parser::new(md_content);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn ensure_blog() -> impl Filter<Extract = (Blog,), Error = warp::Rejection> + Clone {
    warp::path!(String).and_then(|slug: String| async move {
        // Read file if exists
        let file_path = format!("./blogs/{}.md", slug);
        let md_content = match tokio::fs::read_to_string(file_path).await {
            Ok(content) => content,
            Err(_) => return Err(warp::reject::not_found()),
        };

        // Parse front matter
        let (metadata, content) = parse_front_matter(&md_content);

        // Convert markdown to HTML
        let html_content = markdown_to_html(&content);

        match metadata {
            Some(metadata) => Ok(Blog {
                slug,
                metadata,
                content: html_content,
            }),
            None => Err(warp::reject::not_found()),
        }
    })
}
