use maud::{Markup, html};

pub fn template(page: Markup) -> Markup {
    html!(
        head {
            title { "MusiSeerr" }
            script src="/static/htmx.min.js" {}
        }
        body {
            h1 {
                "MusiSeerr"
            }

            content {
                (page)
            }
        }
    )
}