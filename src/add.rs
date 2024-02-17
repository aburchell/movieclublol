use crate::movies;
use crate::reviews;
use maud::{html, Markup};
use sqlx::{Pool, Postgres};

async fn review_is_unique(movie_id: i32, author: String, copy: String, pool: &Pool<Postgres>) -> bool {
    let identical:Option<reviews::Review> = sqlx::query_as!(
        reviews::Review, r#"
        SELECT * FROM reviews
        WHERE movie_id=$1 AND author=$2 AND copy=$3"#, movie_id, author, copy
        ).fetch_optional(pool).await.unwrap();
    match identical {
        Some(_) => false,
        None => true,
    }
}
pub async fn review(movie_id: i32, author: String, copy: String, pool: &Pool<Postgres>) {
    let a = author.clone();
    let c = copy.clone();
    if !review_is_unique(movie_id, a, c, pool).await {return};
    sqlx::query_as!(
        reviews::Review,
        r#"
        INSERT INTO reviews (movie_id, author, copy)
        VALUES ($1, $2, $3)
    "#,
        movie_id,
        author,
        copy
    )
    .fetch_one(pool)
    .await
    .unwrap();
}

pub async fn movie_or_get_id(
    title: String,
    director: Option<String>,
    year: i32,
    pool: &Pool<Postgres>,
) -> i32 {
    let existing_movie: Option<movies::Movie> = sqlx::query_as!(
        movies::Movie,
        r#"
            SELECT * FROM movies
            WHERE title=$1 AND year=$2
            "#,
        title,
        year,
    )
    .fetch_optional(pool)
    .await
    .unwrap();
    println!("{:?}", title);
    println!("{:?}", director);
    let movie_id;
    match existing_movie {
        Some(movie) => {
            println!("Pre-existing movie found, adding reviews to this movie.");
            movie_id = movie.id;
        }
        None => {
            println!(
                "Movie called '{}' released in {} not found. Adding to DB now.",
                title, year
            );
            let t = title.clone();
            crate::add::movie(title, director, year, pool).await;
            let this_movie = movies::from_title_and_year(t, year, pool).await.unwrap();
            movie_id = this_movie.id;
        }
    }
    movie_id
}

pub async fn movie(title: String, director: Option<String>, year: i32, pool: &Pool<Postgres>) {
    // ) -> Result<movies::Movie, Error> {
    let director = director.unwrap_or(String::from(""));
    sqlx::query_as!(
        movies::Movie,
        r#"
            INSERT INTO movies (title, director, year)
            VALUES ($1, $2, $3)
            "#,
        title,
        director,
        year
    )
    .fetch_optional(pool)
    .await
    .unwrap();
}

pub fn panel_html() -> Markup {
    html! {
        iframe name="hacky" style="display:none;" {}
        form ."add-panel" ."box" name="addReviewForm" method="POST" action="/addReview" style="width: 85vw; max-height:35vh; padding: 1em; margin: 1em;" target="hacky"{
            h2 {"Review a film!"}
            ."columns" {
                ."field" ."column" {
                    label ."label" { "Title" }
                    ."control" {
                        input ."input" type="text" name="title" autocomplete="off" required {}
                    }
                }
                ."field" ."column" {
                    label ."label" { "Year" }
                    ."control" {
                        input ."input" type="number" name="year"  autocomplete="off" required {}
                    }
                }
                ."field" ."column" {
                    label ."label" { "Director" }
                    ."control" {
                        input ."input" type="text" name="director" autocomplete="off"{}
                    }
                }
            }
            ."columns"{
                ."field" ."column" ."is-three-quarters"{
                    label ."label" { "Thoughts?" }
                    ."control" {
                        input ."input" type="text" name="copy" autocomplete="off" required {}
                    }
                }
                ."field" ."column"{
                    label ."label" { "Sincerely,"}
                    ."control" {
                        input ."input" type="text" name="author" autocomplete="off" required {}
                    }

                }
            }
            ."field" {
                ."control" {
                    button ."button" ."is-link" onclick="setTimeout(() => document.addReviewForm.reset(), 10); console.log('Reset!')" { "Submit" }
                }
            }
        }
    }
}
