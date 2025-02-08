use axum::response::Html;

pub fn html_page(title: &str, extra_head: &str, body: &str) -> Html<String> {
    Html(format!(
        r##"
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>{title}</title>
        <link rel="stylesheet" href="www/style/simple.css" />
        <script src="www/js/htmx.js"></script>
{extra_head}
    </head>

    <body>
{body}
    </body>
</html>
    "##
    ))
}
