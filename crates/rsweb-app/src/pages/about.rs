use maud::{DOCTYPE, Markup, html};

use crate::components::{
    load_theme::LOAD_THEME,
    nav::{NAV_SCRIPT, navbar},
};

pub fn render() -> Markup {
    html! {
      (DOCTYPE)
      html {
        head {
          title { "About" }
          script defer src="/static/router.js" {}
          (LOAD_THEME)
          (NAV_SCRIPT)
          link data-dynamic rel="stylesheet" type="text/css" href="/static/app.css" {}
        }
        body {
          div id="app" {
            (navbar(None))
            h1 { "About" }
          }
        }
      }
    }
}
