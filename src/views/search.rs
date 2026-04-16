use maud::{Markup, html};

use crate::objects::{SearchParameters, SearchResult, SearchType};

pub fn search_bar(parameters: &SearchParameters) -> Markup {
    html!(
        form class="search-bar text-center" hx-get="./search" hx-target="#search-content" hx-disabled-elt="#search-button" hx-replace-url="true" {
            div class="input-group" {
                select class="form-select w-auto flex-grow-0" placeholder="Type" required name="type" {
                    option value="music" selected[parameters.search_type == Some(SearchType::Music)] {"Music"}
                    option value="album" selected[parameters.search_type == Some(SearchType::Album)] {"Album"}
                    option value="artist" selected[parameters.search_type == Some(SearchType::Artist)] {"Artist"}
                }
                input required class="form-control" name="query" value=(parameters.query.as_ref().unwrap_or(&"".to_string())) placeholder="Search text" {}
                button type="submit" id="search-button" class="btn btn-primary" { "search" }
            }
        }
    )
}

pub fn search_result(result: &SearchResult) -> Markup {
    match result {
        SearchResult::Musics(musics) => {
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
                        @for music in musics {
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
        },
        SearchResult::Artists(artists) => {
            html!{
                h1 { "Artists" }

                table class="table align-middle" {
                    thead class="sticky-top" {
                        tr {
                            th {"Name"}
                            th {"Request"}
                        }
                    }
                    tbody {
                        @for artist in artists {
                            tr { 
                                td {(artist.name)}
                                td {
                                    form hx-post="/request_artist" hx-disabled-elt="find button" hx-target="this" hx-swap="outerHTML" {
                                        button class="btn btn-secondary" type="submit" disabled[artist.monitored] {
                                            "Request"
                                        }
                                        input type="hidden" value=(artist.id) name="artist_id" {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        SearchResult::Albums(albums) => {
            html!{
                h1 { "Albums" }

                table class="table align-middle" {
                    thead class="sticky-top" {
                        tr {
                            th {"Name"}
                            th {"Artist"}
                            th {"Request"}
                        }
                    }
                    tbody {
                        @for album in albums {
                            tr { 
                                td {(album.name)}
                                td {(album.artist)}
                                td {
                                    form hx-post="/request_album" hx-disabled-elt="find button" hx-target="this" hx-swap="outerHTML" {
                                        button class="btn btn-secondary" type="submit" disabled[album.monitored] {
                                            "Request"
                                        }
                                        input type="hidden" value=(album.id) name="album_id" {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}