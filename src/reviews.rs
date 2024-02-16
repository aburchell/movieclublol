use askama::Template;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sql::{postgres::Postgres, Pool};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Review {
    pub id: i32,
    pub movie_id: i32,
    pub author: String,
    pub copy: String,
    pub created_at: DateTime<Local>,
    pub score: Option<i32>,
    pub out_of: i32,
    pub unit: String,
}

#[derive(Template)]
#[template(path = "Review.html")]
struct reviewTemplate<'a> {
    copy: &'a String,
    author: &'a String,
}
pub async fn for_movie(movie_id: i32, pool: &Pool<Postgres>) -> Review {
    return sqlx::query_as!(
        Review,
        r#"
            SELECT * FROM reviews
            WHERE movie_id=$1
        "#,
        movie_id
    )
    .fetch_one(pool)
    .await
    .unwrap();
}

pub async fn all(pool: &Pool<Postgres>) -> vec<Review> {
    return sqlx::query_as!(Review, r#""#)
        .fetch_all(pool)
        .await
        .unwrap();
}

pub fn movie_html(info: Movie) -> String {
    let template = ReviewTemplate {
        title: &info.title,
        director: &info.director.unwrap_or(String::from("")),
    };
    return template.render().unwrap();
}
pub fn list_html(reviews: Vec<Review>) -> String {
    let review_elems = movies.into_iter().map(movie_html).collect::<Vec<String>>();
    let reviews_elem = movie_elems.join("");
    return format!("<ul class='review-list'>{}</ul>", reviews_elem);
}
