use maud::{Markup, html};

use crate::objects::{SearchParameters, SearchResult};

pub fn search_bar(parameters: &SearchParameters) -> Markup {
    html!(
        form hx-get="./search" hx-target="#search-content" hx-disabled-elt="#search-button" hx-replace-url="true" {
            input name="query" value=(parameters.query.as_ref().unwrap_or(&"".to_string())) placeholder="Search text" {}
            button type="submit" id="search-button" { "search" }
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
                    th {"Type"}
                    th {"Artist"}
                    th {"Monitored"}
                    th {"Request"}
                }
            }
            tbody {
                @for music in &result.musics {
                    tr { 
                        td {(music.title)}
                        td {(music.album)}
                        td {(music.album_type)}
                        td {(music.artist)}
                        td {(music.monitored)}
                        td {
                            form hx-post="/request_music" hx-disabled-elt="find button" hx-target="this" hx-swap="outerHTML" {
                                button type="submit" disabled[music.monitored] {
                                    "Request"
                                }
                                input type="hidden" value=(music.id) name="music_id" {}
                            }
                        }
                    }
                }
            }
        }
    }
}