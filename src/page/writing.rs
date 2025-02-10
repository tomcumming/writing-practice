use std::{collections::BTreeMap, path::PathBuf};

use axum::{extract::Query, http::Response, response::Html, routing, Router};
use tower_http::services::ServeDir;

use crate::{
    config,
    template::{self, throw_500},
};

pub fn add_routes(router: Router) -> Router {
    let config = config::load_config().as_ref().expect("Can't load config!");
    router
        .route("/writing", routing::get(writing_root))
        .route("/writing/preview", routing::get(preview))
        .nest_service(
            "/writing/stroke-order-data",
            ServeDir::new(config.stroke_order_data.as_str()),
        )
}

async fn writing_root() -> Html<String> {
    template::html_page(
        "Main page",
        r##"<script type="module" src="www/js/stroke-order.js"></script>"##,
        r##"
        <input type="text"
            name="search"
            hx-trigger="input[!isComposing] delay:100ms, compositionend delay:100ms"
            hx-get="/writing/preview"
            hx-target="#result"
        >
        <div id="result" class="stroke-order-preview"></div>
        "##,
    )
}

async fn preview(
    Query(params): Query<BTreeMap<String, String>>,
) -> Result<Html<String>, Response<String>> {
    let search = params
        .get("search")
        .ok_or(throw_500("'search' not found".to_string()))?;

    let config = config::load_config()
        .as_ref()
        .map_err(Clone::clone)
        .map_err(throw_500)?;

    let characters = search.chars().filter(|c| {
        let mut path = PathBuf::new();
        path.push(&config.stroke_order_data);
        path.push(format!("{c}.json"));
        println!("{:?}", path);
        path.exists()
    });

    let image_tags = characters
        .map(|ch| {
            format!(
                r##"<stroke-order
                character="{}"
                data-path="/writing/stroke-order-data"
            ></stroke-order>"##,
                ch,
            )
        })
        .fold(String::new(), |mut p, c| {
            p.push_str(&c);
            p.push_str("\n");
            p
        });

    Ok(Html(image_tags))
}
