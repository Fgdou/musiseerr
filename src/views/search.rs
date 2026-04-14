use maud::{Markup, html};

use crate::objects::{SearchParameters, SearchResult};

pub fn search_bar(parameters: &SearchParameters) -> Markup {
    html!(
        form class="search-bar text-center row g-2" hx-get="./search" hx-target="#search-content" hx-disabled-elt="#search-button" hx-replace-url="true" {
            div class="input-group" {
                input class="form-control" name="query" value=(parameters.query.as_ref().unwrap_or(&"".to_string())) placeholder="Search text" {}
                button type="submit" id="search-button" class="btn btn-primary" { "search" }
            }
        }
    )
}

pub fn search_result(result: &SearchResult) -> Markup {
    html!{
        h1 { "Musics" }

        table class="table align-middle" {
            thead class="sticky-top" {
                tr {
                    th {"Title"}
                    th {"Album"}
                    th {"Artist"}
                    th {"Request"}
                }
            }
            tbody {
                @for music in &result.musics {
                    tr { 
                        td {(music.title)}
                        td {(music.album)}
                        td {(music.artist)}
                        td {
                            form hx-post="/request_music" hx-disabled-elt="find button" hx-target="this" hx-swap="outerHTML" {
                                button class="btn btn-secondary" type="submit" disabled[music.monitored] {
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