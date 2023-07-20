use std::fs::{self, FileType, ReadDir};

use axum::{extract::Path, http::StatusCode, response::Response, routing::get, Router};
use chain_tools::*;
use std::fs::File;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/bigger/luke/*path", get(serve_dir))
        .route("/bigger/luke/:path", get(serve_dir))
        .route("/bigger/luke", get(|| serve_dir(None)));

    axum::Server::bind(&"0.0.0.0:1971".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_dir(path: Option<Path<String>>) -> Result<Response<String>, StatusCode> {
    let path_str = format!("dist/{}", path.map(|Path(p)| p).unwrap_or_default());
    let path = std::path::Path::new(&path_str);

    let path_str = if path.is_dir() {
        path_str.clone() + "/index.html"
    } else {
        path_str.clone()
    };

    println!("{path_str}");

    let ext = std::path::Path::new(&path_str)
        .extension()
        .map(|oss| oss.to_str())
        .unwrap()
        .unwrap();
    let entry = fs::read_to_string(&path_str).map_err(|_| StatusCode::BAD_REQUEST)?;

    println!("{path_str}: {ext}");

    let content_type = match ext {
        "html" => "text/html",
        "js" => "text/javascript",
        "css" => "text/css",
        o => {
            println!("{o}");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let res = Response::builder()
        .header("Content-Type", content_type)
        .body(entry)
        .unwrap();

    Ok(res)
}
