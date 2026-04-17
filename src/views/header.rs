use maud::{Markup, html};

pub fn template(page: Markup) -> Markup {
    html!(
        head {
            title { "MusiSeerr" }
            link href="/static/bootstrap.min.css" rel="stylesheet" {}
            meta name="viewport" content="width=device-width, initial-scale=1" {}
        }
        body {
            h1 class="text-center" {
                "MusiSeerr"
            }

            div class="text-center" {
                "Request any music / album / artist to be monitored"
            }

            div class="container mt-5" {
                (page)
            }

            script src="/static/htmx.min.js" {}
            script src="/static/bootstrap.min.js" {}
        }
    )
}