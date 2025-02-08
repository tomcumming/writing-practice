use std::collections::BTreeMap;

use axum::{extract::Query, http::Response, response::Html, routing, Router};

use crate::template;

pub fn add_routes(router: Router) -> Router {
    router
        .route("/writing", routing::get(writing_root))
        .route("/writing/preview", routing::get(preview))
}

async fn writing_root() -> Html<String> {
    template::html_page(
        "Main page",
        "",
        r##"
        <input type="text"
            name="search"
            hx-trigger="input[!isComposing] delay:100ms, compositionend delay:100ms"
            hx-get="/writing/preview"
            hx-target="#result"
        >
        <h4 id="result"></h4>
        </script>
        "##,
    )
}

async fn preview(
    Query(params): Query<BTreeMap<String, String>>,
) -> Result<Html<String>, Response<String>> {
    let search = params.get("search").ok_or(
        Response::builder()
            .status(500)
            .body("'search' not found".to_owned())
            .unwrap(),
    )?;
    Ok(Html(format!("TODO {search}")))
}
