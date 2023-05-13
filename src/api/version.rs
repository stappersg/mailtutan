use axum::response::Html;

pub async fn show() -> Html<&'static str> {
    Html(env!("CARGO_PKG_VERSION"))
}