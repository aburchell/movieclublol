use maud::{html, PreEscaped, Markup};
use dotenv::dotenv;
use poem::{get, post, handler, listener::TcpListener, web::Data, web::Path, EndpointExt, Route, Server, Response, http::StatusCode, Request, Body, web::Form};
use sqlx::{Pool, postgres::{PgPoolOptions, Postgres}};
use serde::Deserialize;

mod movies;
mod reviews;
mod add;
mod etc;

// Review handlers
#[handler]
async fn get_review(Path(id): Path<i32>, data: Data<&Pool<Postgres>>) -> String {
    let pool: &Pool<Postgres> = data.0;
    let review = reviews::from_id(id, pool).await;
    return reviews::review_html(review).into_string();
}
#[handler]
async fn get_all_reviews_for_movie(Path(movie_id): Path<i32>, data:Data<&Pool<Postgres>>) -> String {
    let pool: &Pool<Postgres> = data.0;
    let reviews = reviews::for_movie(movie_id, pool).await;
    return reviews::list_html(reviews).into_string();
}

// Movie handlers
#[handler]
async fn get_movie(Path(id): Path<i32>, data: Data<&Pool<Postgres>>) -> String {
    let pool: &Pool<Postgres> = data.0;
    let movie = movies::from_id(id, pool).await;
    return movies::movie_html(movie).into_string();
}
#[handler]
async fn get_all_movies(data: Data<&Pool<Postgres>>) -> String {
    let pool: &Pool<Postgres> = data.0;
    let movies = movies::all(pool).await;
    return movies::list_html(movies).into_string();
}

// Add-review handlers
#[derive(Deserialize)]
struct CreateReview {
    title: String, director: Option<String>, copy: String
}
#[handler]
async fn add_review(Form(CreateReview {title, director, copy }): Form<CreateReview>, data: Data<&Pool<Postgres>>) -> String {
    let pool: &Pool<Postgres> = data.0;
    
    println!("{:?}", title);
    println!("{:?}", director);
    println!("{:?}", copy);
    return String::from("")
}

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let app = Route::new()
        .at("/", get(index))
        .at("/addReview", post(add_review))
        .data(pool);

    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await;
}

#[handler]
async fn index(data: Data<&Pool<Postgres>>) -> Response {
    let pool: &Pool<Postgres> = data.0;
    let movies = movies::list_html(movies::all(pool).await);
    let add_panel = add::panel_html();
    let header = etc::header();
    let markup = html! {
        (header)
        body {
            (movies)
            (add_panel)
        }
    };
    let home_html = markup.into_string();
    return Response::builder()
        .status(StatusCode::OK)
        .body(home_html);
}

