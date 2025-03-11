use rsweb_auth::claims::Claims;
use maud::{DOCTYPE, Markup, html};

use crate::components::{
    load_theme::LOAD_THEME,
    nav::{NAV_SCRIPT, navbar},
};

pub async fn home(claims: Option<&Claims>) -> Markup {
    html! {
      (DOCTYPE)
      html {
        head {
          title { "Home" }
          script defer src="/static/router.js" {}
          (LOAD_THEME)
          (NAV_SCRIPT)
          link data-dynamic rel="stylesheet" type="text/css" href="/static/app.css" {}
          style data-dynamic { r#"
              .hero {
                  position: relative;
                  height: 80vh;

                  img {
                        width: 100%;
                        height: 100%;
                        object-fit: cover;
                  }
              }
              .overlay-fade {
                  z-index: 10;
                  position: absolute;
                  top: 0;
                  right: 0;
                  bottom: 0;
                  left: 0;
                  background-image: linear-gradient(to right, rgb(0 0 0 / .7), rgb(0 0 0 / .3));
              }
          "# }
        }
        body {
          div id="app" {
            (navbar(claims))
            main {
                div class="hero" {
                    img src="/static/images/hero.jpg";
                    div class="overlay-fade";
                }
                h1 { "Welcome!" }
            }
          }
        }
      }
    }
}
