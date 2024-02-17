use crate::movies;
use maud::{html, Markup};
use sqlx::{Pool, Postgres};

pub async fn review(movie_id: i32, author: String, copy: String, pool: &Pool<Postgres>) {
    sqlx::query_as!(
        crate::reviews::Review,
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
        form ."add-panel" ."box" name="addReviewForm" method="POST" action="/addReview" style="position: fixed; bottom: 10px; padding: 1em; margin: 1em;" target="hacky"{
            h2 {"Review a film!"}
            ."field" {
                label ."label" { "Title" }
                ."control" {
                    input ."input" type="text" name="title" autocomplete="off" required {}
                }
            }
            ."field" {
                label ."label" { "Year" }
                ."control" {
                    input ."input" type="number" name="year"  autocomplete="off" required {}
                }
            }
            ."field" {
                label ."label" { "Director" }
                ."control" {
                    input ."input" type="text" name="director" autocomplete="off"{}
                }
            }
            ."field" {
                label ."label" { "Thoughts?" }
                ."control" {
                    input ."input" type="text" name="copy" autocomplete="off" required {}
                }
            }
            ."field" {
                label ."label" { "Sincerely,"}
                ."control" {
                    input ."input" type="text" name="author" autocomplete="off" required {}
                }

            }
            ."field" {
                ."control" {
                    button ."button" ."is-link" onclick="document.addReviewForm.submit(); setTimeout(() => document.addReviewForm.reset(), 10); console.log('Reset!')" { "Submit" }
                }
            }
        }
    }
}
