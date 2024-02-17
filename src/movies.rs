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

pub async fn from_title_and_year(title: String, year: i32, pool: &Pool<Postgres>) -> Option<Movie> {
    let movie = sqlx::query_as!(
        Movie,
        r#"
        SELECT * FROM movies
        WHERE title=$1 AND year=$2
    "#,
        title,
        year
    )
    .fetch_optional(pool)
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

pub async fn movie_html(info: Movie, pool: &Pool<Postgres>) -> Markup {
    let reviews:Vec<crate::reviews::Review> = crate::reviews::for_movie(info.id, pool).await;
    html! {
        ."movie-card" ."card" style="padding: 1em; margin: 2em;" {
            ."hero" . "is-info" {
                h1  ."title" { (info.title) }
                @if info.director.is_some() {
                    h2 ."movie-director" ."subtitle" { (info.director.unwrap()) }
                }
            }
            @if !reviews.is_empty() {
                ul ."review-list" {
                    @for review in reviews {
                        li ."review-item" {
                            (crate::reviews::review_html(review))
                        }
                    }
                }
            }
        }
    }
}
pub async fn list_html(movies: Vec<Movie>, pool: &Pool<Postgres>) -> Markup {
    html! {
        @if !movies.is_empty() {
            ul ."movie-list" {
                @for movie in movies {
                    li ."movie-list-item" {
                        (movie_html(movie, pool).await)
                    }
                }
            }
        } @else {
            "No movies reviewed yet, please change that."
        }
    }
}
