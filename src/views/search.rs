use maud::{Markup, html};

use crate::objects::{SearchParameters, SearchResult};

pub fn search_bar(parameters: &SearchParameters) -> Markup {
    html!(
        form action="./search" method="get" {
            input name="query" value=(parameters.query.as_ref().unwrap_or(&"".to_string())) placeholder="Search text" {}
            button type="submit" { "search" }
        }
    )
}

pub fn search_result(result: &SearchResult) -> Markup {
    html!{
        h1 { "Musics" }

        table {
            thead {
                tr {
                    th {"Title"}
                    th {"Album"}
                    th {"Artist"}
                }
            }
            tbody {
                @for music in &result.musics {
                    tr { 
                        td {(music.title)}
                        td {(music.album)}
                        td {(music.artist)}
                    }
                }
            }
        }
    }
}