use std::net::SocketAddr;

use axum::{Router, routing::get};
use listenfd::ListenFd;
use maud::{DOCTYPE, Markup, html};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn base() -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "yanandog.training" }
            meta charset="utf-8";
            meta name="color-scheme" content="light";
            link rel="stylesheet" href ="/assets/missing.css";
            link rel="stylesheet" href ="/assets/style.css";
            link rel="icon" type="image/x-icon" href="/assets/favicon.ico";
            script src="/assets/htmx.js" { }
        }
        body {
            header {
                h1 class="flex-row crowded" {
                    img src="/assets/favicon.ico";
                    span class="flex-column packed" {
                        span class="allcaps" { "yana and dog(s)" }
                        sub-title { "trainging and fun"}
                    }
                }
            }
            main {

            }
            footer {
                p { "Made with <3" }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into());

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let app = Router::new()
        .route("/", get(base))
        .nest_service("/assets", ServeDir::new("assets"));

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        None => TcpListener::bind(addr).await.unwrap(),
    };

    tracing::info!("listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
