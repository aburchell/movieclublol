use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use sqlx::postgres::Postgres;
use sqlx::Pool;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct Movie {
    pub id: i32,
    pub title: String,
    pub director: Option<String>,
    pub year: i32,
}

pub async fn from_id(id: i32, pool: &Pool<Postgres>) -> Movie {
    let movie = sqlx::query_as!(
        Movie,
        r#"
        SELECT * FROM movies
        WHERE id=$1
    "#,
        id
    )
    .fetch_one(pool)
    .await
    .unwrap();
    return movie;
}

pub async fn all(pool: &Pool<Postgres>) -> Vec<Movie> {
    return sqlx::query_as!(
        Movie,
        r#"
            SELECT * FROM movies
        "#
    )
    .fetch_all(pool)
    .await
    .unwrap();
}

pub fn movie_html(info: Movie) -> Markup {
    html! {
        ."movie-card" {
            ."hero" . "is-info" {
                h1  ."title" { (info.title) }
                @if info.director.is_some() {
                    h2 ."movie-director" ."subtitle" { (info.director.unwrap()) }
                }
            }
        }
    }
}
pub fn list_html(movies: Vec<Movie>) -> Markup {
    html! {
        @if !movies.is_empty() {
            ul ."movie-list" {
                @for movie in movies {
                    li ."movie-list-item" {
                        (movie_html(movie))
                    }
                }
            }
        } @else {
            "No movies reviewed yet, please change that."
        }
    }
}
