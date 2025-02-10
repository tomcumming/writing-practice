use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::Response,
    response::Html,
    routing, Router,
};

use crate::{
    db,
    template::{self, throw_500},
};

pub fn add_routes(router: Router) -> Router {
    router.route("/import", routing::get(import_root)).route(
        "/import/cc-cedict",
        routing::post(upload_cccedict).layer(DefaultBodyLimit::max(20_000_000)),
    )
}

async fn import_root() -> Html<String> {
    template::html_page(
        "Import Data",
        "",
        r##"
        <h2>Import CC-CEdict</h2>
        <form method="post" enctype="multipart/form-data" action="/import/cc-cedict">
            <label>CC-CEdict text file:</label>
            <input type="file" name="cc-cedict">
            <input type="submit" value="Upload">
        </form>
        "##,
    )
}

async fn upload_cccedict(mut multipart: Multipart) -> Result<String, Response<String>> {
    let field = multipart.next_field().await.unwrap().unwrap();

    if field.name() != Some("cc-cedict") {
        Err(throw_500("Wrong field name".to_string()))?
    }

    let contents = field.text().await.unwrap();

    if multipart.next_field().await.unwrap().is_some() {
        Err(throw_500("Expected one field!".to_string()))?
    }

    let lines_res: Result<Vec<db::WordDef>, String> = contents
        .lines()
        .enumerate()
        .skip_while(|(_idx, line)| line.starts_with("#"))
        .map(|(idx, line)| {
            parse_cc_cedict_line(line).map_err(|e| format!("On line {}: {}", idx, e))
        })
        .collect();
    let lines = lines_res.map_err(throw_500)?;

    Ok(format!("{:?}", lines[5000]))
}

fn parse_cc_cedict_line(line: &str) -> Result<db::WordDef, String> {
    let remaining = line;

    let (traditional, remaining) = remaining
        .split_once(" ")
        .ok_or("Could not read simplified")?;
    let (simplified, remaining) = remaining
        .split_once(" ")
        .ok_or("Could not read simplified")?;
    let remaining = remaining.strip_prefix("[").ok_or("Expected [")?;
    let (pinyins, remaining) = remaining.split_once("] /").ok_or("Could not read pinyin")?;
    let defs = remaining
        .split("/")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(db::WordDef {
        simplified: simplified.to_string(),
        traditional: traditional.to_string(),
        pinyin: pinyins.split_whitespace().map(|s| s.to_string()).collect(),
        defs,
    })
}
