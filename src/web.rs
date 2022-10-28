mod middleware;

use maud::{html, DOCTYPE};
use poem::listener::TcpListener;
use poem::middleware::CatchPanic;
use poem::web::{Html, Json};
use poem::{get, handler, EndpointExt, IntoResponse, Route, Server};

use crate::persistence::Db;
use crate::prelude::*;
use crate::web::middleware::*;

pub async fn run(endpoint: String, db: Db) -> Result {
    let app = Route::new()
        .at("/", get(index))
        .at("/health", get(health))
        .data(db)
        .with(CatchPanic::new())
        .with(ErrorMiddleware);
    Server::new(TcpListener::bind(endpoint))
        .run(app)
        .await
        .context("the server has failed")
}

#[handler]
async fn health() -> impl IntoResponse {
    Json(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

#[handler]
async fn index() -> Result<impl IntoResponse> {
    let markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "meterkastd" }
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css";
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma-prefers-dark@0.1.0-beta.1/css/bulma-prefers-dark.min.css" crossorigin="anonymous" referrerpolicy="no-referrer";
                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.2.0/css/all.min.css" integrity="sha512-xh6O/CkQoPOWDdYTDqeRdPCVd1SpvCA9XXcUnZS2FmJNp1coAFzvtCN9BmamE+4aHK8yyUHUSCcJHgXloTyT2A==" crossorigin="anonymous" referrerpolicy="no-referrer";
            }

            body {
                section.hero.is-fullheight {
                    div.hero-head {
                        nav.navbar.has-shadow {
                            div.container {
                                div.navbar-brand {
                                    a.navbar-item.has-text-weight-bold href="/" {
                                        span.icon { i.fa-solid.fa-gauge {} }
                                        span { "meterkastd" }
                                    }
                                }
                            }
                        }
                    }

                    div.hero-body {
                        div.container {
                            div.columns {
                                div.column {
                                    div.card {
                                        header.card-header {
                                            p.card-header-title {
                                                span.icon-text.is-flex-wrap-nowrap {
                                                    span.icon.has-text-warning-dark { i.fa-solid.fa-bolt {} }
                                                    span { "Electricity" }
                                                }
                                            }
                                        }

                                        div.card-content {
                                            nav.level {
                                                div.level-item.has-text-centered {
                                                    div {
                                                        p.heading { span."is-size-6" { "Per hour" } }
                                                        p.title."is-1" { "0.5 " span."is-size-3" { "kW" } }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                div.column {
                                    div.card {
                                        header.card-header {
                                            p.card-header-title {
                                                span.icon-text.is-flex-wrap-nowrap {
                                                    span.icon.has-text-danger { i.fa-solid.fa-fire {} }
                                                    span { "Gas" }
                                                }
                                            }
                                        }

                                        div.card-content {
                                            nav.level {
                                                div.level-item.has-text-centered {
                                                    div {
                                                        p.heading { span."is-size-6" { "Per hour" } }
                                                        p.title."is-1" { "0.10 " span."is-size-3" { "㎥" } }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div.hero-foot {}
                }
            }
        }
    };
    Ok(Html(markup.into_string()))
}
