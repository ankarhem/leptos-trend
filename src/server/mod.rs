mod generated;

// use crate::app::*;
// use actix_files::Files;
use actix_web::*;
// use futures::StreamExt;
use leptos::*;
use leptos_trend::app::*;
use std::{env, net::SocketAddr};

pub async fn run() -> std::io::Result<()> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    log::info!("serving at {host}:{port}");

    HttpServer::new(move || {
        let render_options: RenderOptions = RenderOptions::builder()
            .pkg_path("/pkg")
            .reload_port(3001)
            .socket_address(addr.clone())
            .environment(&env::var("RUST_ENV"))
            .build();
        App::new()
            // .service(
            //     web::scope("/pkg")
            //         .service(Files::new("", "target/site/pkg"))
            //         .wrap(middleware::Compress::default()),
            // )
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .route(
                "{tail:.*}",
                leptos_actix::render_app_to_stream(render_options, |cx| view! { cx, <App /> }),
            )
        // .service(render_app)
    })
    .bind((host, port))?
    .run()
    .await
}
