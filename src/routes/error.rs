use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate<'a> {
    url: &'a str,
    page_title: &'a str,
    error: &'a str,
}

// Render error template
pub fn render_error(url: &str, error: &str) -> Html<String> {
    // Get .env variables
    let page_title = std::env::var("PAGE_TITLE").unwrap_or_else(|_| "files".to_string());

    // Render template
    let error_template = ErrorTemplate {
        url,
        page_title: &page_title,
        error,
    };

    error_template
        .render()
        .map(Html)
        .unwrap_or_else(|_| Html("Error rendering error template".to_string()))
}
