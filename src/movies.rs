use askama::Template;
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

#[derive(Template)]
#[template(path = "Movie.html")]
struct MovieTemplate<'a> {
    title: &'a String,
    director: &'a String,
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

pub fn movie_html(info: Movie) -> String {
    let template = MovieTemplate {
        title: &info.title,
        director: &info.director.unwrap_or(String::from("")),
    };
    return template.render().unwrap();
}
pub fn list_html(movies: Vec<Movie>) -> String {
    let movie_elems = movies.into_iter().map(movie_html).collect::<Vec<String>>();
    let movies_elem = movie_elems.join("");
    return format!("<ul class='movie-list'>{}</ul>", movies_elem);
}
