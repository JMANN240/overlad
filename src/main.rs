use std::{fmt::Debug, io::Cursor, path::PathBuf};

use ab_glyph::FontRef;
use axum::{
    extract::{DefaultBodyLimit, Query},
    http::{header::CONTENT_TYPE, HeaderMap},
    response::IntoResponse,
    routing::{get, post},
    serve::Listener,
};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use clap::{Args, Parser};
use current_previous::CurrentPrevious;
use image::{ImageFormat, Rgb};
use imageproc::drawing::text_size;
use maud::{DOCTYPE, Markup, html};
use serde::Deserialize;
use tokio::net::{TcpListener, UnixListener};
use tower_http::services::ServeDir;

use crate::util::draw_text_outline_mut;

mod util;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    listen: Listen,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Listen {
    #[arg(short, long, group = "listen")]
    port: Option<u16>,

    #[arg(short, long, group = "listen")]
    uds: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(port) = cli.listen.port {
        serve_with_listener(TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap()).await;
    } else if let Some(path) = cli.listen.uds {
        let _ = tokio::fs::remove_file(&path).await;
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();

        let listener = UnixListener::bind(path.clone()).unwrap();

        serve_with_listener(listener).await;
    }
}

async fn serve_with_listener<L>(listener: L)
where
    L: Listener,
    L::Addr: Debug,
{
    let app = axum::Router::new()
        .route("/", get(root))
        .route("/api", post(overlay))
        .fallback_service(ServeDir::new("static"))
        .layer(DefaultBodyLimit::max(8000000));

    axum::serve(listener, app).await.unwrap();
}

pub fn page(main: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                link rel="stylesheet" href="/styles.css";
            }
            body {
                header {
                    nav {
                        h1 { "OverLad" }
                    }
                }
                main {
                    (main)
                }
            }
        }
    }
}

pub async fn root() -> Markup {
    page(html! {
        h1 { "Hello, OverLad!" }
    })
}

#[derive(TryFromMultipart)]
pub struct OverlayMultipart {
    text: String,
    image: axum::body::Bytes,
}

#[derive(Deserialize)]
pub struct OverlayQuery {
    #[serde(default)]
    thickness: f64,
}

pub async fn overlay(Query(query): Query<OverlayQuery>, multipart: TypedMultipart<OverlayMultipart>) -> impl IntoResponse {
    let dynamic_image = image::load_from_memory(&multipart.image).unwrap();
    let mut image = dynamic_image.into_rgb8();

    let image_min = image.width().min(image.height());
    let margin = image_min as f64 * 0.05;

    let words = multipart.text.split(" ").collect::<Vec<&str>>();
    let font = FontRef::try_from_slice(include_bytes!("../roboto.ttf")).unwrap();
    let font_scale = image_min as f32 * 0.2;

    let max_width = image.width() as f64 * 0.75 - 2.0 * margin;

    let thickness = query.thickness * image_min as f64 * 0.001;
    let mut line_words = CurrentPrevious::new(Vec::new());
    let mut y_offset = 0;
    for word in words {
        let mut new_line_words = line_words.current().clone();
        new_line_words.push(word);

        line_words.update(new_line_words);

        let current_line = line_words.current().join(" ");
        let current_measurement = text_size(font_scale, &font, &current_line);

        if let Some(previous_line_words) = line_words.previous() {
            let previous_line = previous_line_words.join(" ");
            let previous_measurement = text_size(font_scale, &font, &previous_line);

            if (current_measurement.0 as f64) > max_width && (previous_measurement.0 as f64) < max_width
            {
                draw_text_outline_mut(
                    &mut image,
                    Rgb([255, 255, 255]),
                    Rgb([0, 0, 0]),
                    thickness,
                    margin as i32,
                    margin as i32 + y_offset,
                    font_scale,
                    &font,
                    &previous_line,
                );
                
                line_words.update(vec![line_words.current().last().unwrap()]);
                y_offset += font_scale as i32;
            }
        }
    }

    let current_line = line_words.current().join(" ");
    draw_text_outline_mut(
        &mut image,
        Rgb([255, 255, 255]),
        Rgb([0, 0, 0]),
        thickness,
        margin as i32,
        margin as i32 + y_offset,
        font_scale,
        &font,
        &current_line,
    );

    let mut buf = Cursor::new(Vec::new());
    image.write_to(&mut buf, ImageFormat::Png).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "image/png".parse().unwrap());

    (headers, buf.into_inner())
}

