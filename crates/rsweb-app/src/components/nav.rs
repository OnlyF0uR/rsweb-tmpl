use maud::{PreEscaped, html};
use rsweb_auth::claims::Claims;

// Styles for navbar are in app.css
pub fn navbar(_claims: Option<&Claims>) -> PreEscaped<String> {
    PreEscaped(
        html! {
          div class="navbar" {
            div class="nav-content" {
              a class="nav-logo" href="/" {
                "rsweb"
              }
              nav class="nav-links" {
                  a href="/about" { "About" }
                  a href="/blog/welcome-to-webrs" { "Blog" }
                  a href="/authenticated" { "Authenticated" }
                  a href="https://github.com/OnlyF0uR/rsweb-tmpl" { "Template" }
              }
              div class="nav-actions" {

              }
            }
          }
        }
        .into_string(),
    )
}

// scroll navbar
pub const NAV_SCRIPT: PreEscaped<&'static str> = PreEscaped(
    r#"
    <script>
        window.addEventListener('scroll', () => {
            const nav = document.querySelector('.navbar');
            if (window.scrollY > 0) {
                nav.classList.add('scrolled');
            } else {
                nav.classList.remove('scrolled');
            }
        });
    </script>
    "#,
);
