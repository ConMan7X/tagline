use reqwest::{header, Client, Url};
use serde::Deserialize;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize, Debug)]
struct TmdbMovie {
    title: String,
    tagline: String,
    release_date: String,         // "YYYY-MM-DD"
    genres: Vec<TmdbGenre>,
}

#[derive(Deserialize, Debug)]
struct TmdbGenre {
    name: String,
}

#[derive(Deserialize, Debug)]
struct TmdbCredits {
    cast: Vec<TmdbCast>,
    crew: Vec<TmdbCrew>,
}

#[derive(Deserialize, Debug)]
struct TmdbCast {
    name: String,
    order: u32,
}

#[derive(Deserialize, Debug)]
struct TmdbCrew {
    name: String,
    job: String,
}

#[derive(Deserialize, Debug)]
pub struct Movie {
    pub title: String,
    pub tagline: String,
    pub year: u16,
    pub director: String,
    pub lead_actors: Vec<String>,
    pub genre: String,
}

pub async fn fetch_movie_data(id: &str) -> Result<Movie, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let client = Client::new();

    // Fetch main movie details
    let movie_url = Url::parse_with_params(
        &format!("https://api.themoviedb.org/3/movie/{}", id),
        &[("language", "en-US")],
    )?;


    let tmdb_movie: TmdbMovie = client
        .get(movie_url)
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    // Fetch credits (director + cast)
    let credits_url = Url::parse_with_params(
        &format!("https://api.themoviedb.org/3/movie/{}/credits", id),
        &[("language", "en-US")],
    )?;

    let tmdb_credits: TmdbCredits = client
        .get(credits_url)
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    // Extract year from "YYYY-MM-DD"
    let year = tmdb_movie.release_date
        .split('-')
        .next()
        .and_then(|y| y.parse::<u16>().ok())
        .unwrap_or(0);

    // Find director from crew
    let director = tmdb_credits.crew
        .into_iter()
        .find(|c| c.job == "Director")
        .map(|c| c.name)
        .unwrap_or_else(|| "Unknown".into());

    // Top 3 billed cast members, sorted by order
    let mut cast = tmdb_credits.cast;
    cast.sort_by_key(|c| c.order);
    let lead_actors: Vec<String> = cast.into_iter().take(3).map(|c| c.name).collect();

    // Join genre names into a comma-separated string
    let genre = tmdb_movie.genres
        .iter()
        .map(|g| g.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    Ok(Movie {
        title: tmdb_movie.title,
        tagline: tmdb_movie.tagline,
        year,
        director,
        lead_actors,
        genre,
    })
}