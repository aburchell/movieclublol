use chrono::{DateTime, Local};
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use sqlx::{postgres::Postgres, Pool};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Review {
    pub id: i32,
    pub movie_id: i32,
    pub author: String,
    pub copy: String,
    pub created_at: DateTime<Local>,
    pub score: Option<BigDecimal>,
    pub out_of: Option<BigDecimal>,
    pub unit: Option<String>,
}

pub async fn from_id(review_id: i32, pool: &Pool<Postgres>) -> Review {
    return sqlx::query_as!(
        Review,
        r#"
            SELECT * FROM reviews
            WHERE id=$1
        "#,
        review_id
    )
    .fetch_one(pool)
    .await
    .unwrap();
}
pub async fn for_movie(movie_id: i32, pool: &Pool<Postgres>) -> Vec<Review> {
    return sqlx::query_as!(
        Review,
        r#"
            SELECT * FROM reviews
            WHERE movie_id=$1
        "#,
        movie_id
    )
    .fetch_all(pool)
    .await
    .unwrap_or(vec![]);
}

pub fn review_html(info: Review) -> Markup {
    html! {
       ."review-card" {
           ."review-copy" { (info.copy) }
           ."review-author" { (info.author) }
       }
    }
}
pub fn list_html(reviews: Vec<Review>) -> Markup {
    html! {
        @if !reviews.is_empty() {
            ul ."reviews-list" {
                @for review in reviews {
                    li ."review-list-item" {
                        (review_html(review))
                    }
                }
            }
        } @else {
            "No reviews yet, please add your thoughts!"
        }
    }
}
