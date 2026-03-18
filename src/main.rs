mod tmdb;
mod cli;

use axum::{
    routing::{get},
    Router,
};
use tower_http::services::ServeDir;
use std::env;
use std::net::SocketAddr;
use rand::prelude::IndexedRandom;
use serde::Deserialize;

#[derive(Deserialize)]
struct MovieList {
    movies: Vec<u32>,
}

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "cli" {
        cli::cli(get_random_movie_id()).await;
    } else {
        server().await;
    }
}

fn get_random_movie_id() -> u32 {
    let json = std::fs::read_to_string("data/movies.json").unwrap_or_else(|e| {
        eprintln!("Failed to read movies.json: {}", e);
        std::process::exit(1);
    });

    let movie_list: MovieList = serde_json::from_str(&json).unwrap_or_else(|e| {
        eprintln!("Failed to parse movies.json: {}", e);
        std::process::exit(1);
    });

    *movie_list.movies.choose(&mut rand::rng()).unwrap_or_else(|| {
        eprintln!("Movie list is empty");
        std::process::exit(1);
    })
}

async fn server() {
    println!("Starting server...");

    let app = Router::new()
        .route("/api/movie", get(get_movie_data))
        .fallback_service(ServeDir::new("frontend/dist"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_movie_data() -> Result<axum::Json<tmdb::Movie>, axum::http::StatusCode> {
    let id = get_random_movie_id();
    match tmdb::fetch_movie_data(&id.to_string()).await {
        Ok(movie) => Ok(axum::Json(movie)),
        Err(e) => {
            eprintln!("Error fetching movie data: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}