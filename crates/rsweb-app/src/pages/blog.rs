use crate::{components::load_theme::LOAD_THEME, filters::blog::Blog};
use chrono::NaiveDate;
use maud::{html, Markup, PreEscaped, DOCTYPE};

pub async fn render(content: &Blog) -> Markup {
    let datetime = match content.metadata.date.parse::<NaiveDate>() {
        Ok(dt) => dt.format("%b %d, %Y").to_string(),
        Err(e) => {
            println!("Error parsing blog date ({}): {}", content.slug, e);
            content.metadata.date.clone()
        }
    };

    html! {
      (DOCTYPE)
      html {
        head {
          title { "Blog" }
          script defer src="/static/router.js" {}
          (LOAD_THEME)
          link data-dynamic rel="stylesheet" type="text/css" href="/static/app.css" {}
          style data-dynamic { r#"
            #app {
              display: flex;
              min-height: 100vh;
              align-items: center;
              padding: 2rem;
              flex-direction: column;
            }

            .title {
              font-weight: 700;
              font-size: 3.75rem;
              line-height: 1;
              letter-spacing: -.025em;
              max-width: 56rem;
              color: var(--text-primary);
              margin-top: 5rem;
              text-align: center;
            }

            .subtitle {
              color: var(--text-secondary);
              font-size: 1.375rem;
              line-height: 2rem;
              margin-top: 1.5rem;
              text-align: center;
            }

            .meta {
              color: var(--text-tertiary);
              font-size: 1rem;
              line-height: 1.5rem;
              margin-top: 1.5rem;
            }

            .seperator {
              opacity: .8;
              margin-left: .375rem;
              margin-right: .375rem;
              color: var(--text-tertiary);
            }

            .hero-img {
              border-radius: 0.75rem;
              width: 1200px;
              height: 675px;
              object-fit: cover;
              margin-top: 4rem;
            }

            .content {
              max-width: 48rem;
              margin-top: 7rem;

              h1 {
                font-size: 1.66667em;
                letter-spacing: -.025em;
                line-height: 1.33333;
                margin-bottom: 1.06667em;
                margin-top: 1.86667em;
                color: var(--text-primary);
              }

              h2 {
                font-size: 1.33333em;
                letter-spacing: -.025em;
                line-height: 1.33333;
                margin-bottom: 1.06667em;
                margin-top: 1.86667em;
                color: var(--text-primary);
              }

              p {
                font-size: 18px;
                line-height: 1.75;
                margin-bottom: 1.33333em;
                color: var(--text-primary);
              }
            }
            "# }
        }
        body {
          div id="app" {
            h1 class="title" { (content.metadata.title) }
            p class="subtitle" { (content.metadata.subtitle) }
            p class="meta" {
              time datetime=(content.metadata.date) { (datetime) }
              span class="seperator" { "—" }
              strong { (content.metadata.author) }
            }
            @if let Some(hero_img) = &content.metadata.hero_img {
              img class="hero-img" src=(hero_img) alt="Hero image" {}
            }
            div class="content" {
              (PreEscaped(&content.content))
            }
          }
        }
      }
    }
}
