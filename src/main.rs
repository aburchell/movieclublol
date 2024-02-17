use maud::html;
use poem::{
    get, handler, http::StatusCode, listener::Listener, listener::TcpListener, post, web::Data,
    web::Form, web::Path, EndpointExt, Response, Route, Server,
};
use serde::Deserialize;
use sqlx::{
    postgres::{PgPoolOptions, Postgres},
    Pool,
};

mod add;
mod etc;
mod movies;
mod reviews;

// Review handlers
#[handler]
async fn get_review(Path(id): Path<i32>, data: Data<&Pool<Postgres>>) -> String {
    let pool: &Pool<Postgres> = data.0;
    let review = reviews::from_id(id, pool).await;
    return reviews::review_html(review).into_string();
}
#[handler]
async fn get_all_reviews_for_movie(
    Path(movie_id): Path<i32>,
    data: Data<&Pool<Postgres>>,
) -> String {
    let pool: &Pool<Postgres> = data.0;
    let reviews = reviews::for_movie(movie_id, pool).await;
    return reviews::list_html(reviews).into_string();
}

// Movie handlers
#[handler]
async fn get_movie(Path(id): Path<i32>, data: Data<&Pool<Postgres>>) -> String {
    let pool: &Pool<Postgres> = data.0;
    let movie = movies::from_id(id, pool).await;
    return movies::movie_html(movie, pool).await.into_string();
}
#[handler]
async fn get_all_movies(data: Data<&Pool<Postgres>>) -> String {
    let pool: &Pool<Postgres> = data.0;
    let movies = movies::all(pool).await;
    return movies::list_html(movies, pool).await.into_string();
}

// Add-review handlers
#[derive(Deserialize)]
struct CreateReview {
    title: String,
    year: i32,
    director: Option<String>,
    author: String,
    copy: String,
}
#[handler]
async fn add_review(
    Form(CreateReview {
        title,
        year,
        director,
        author,
        copy,
    }): Form<CreateReview>,
    data: Data<&Pool<Postgres>>,
) {
    let pool: &Pool<Postgres> = data.0;
    let movie_id: i32 = add::movie_or_get_id(title, director, year, pool).await;
    add::review(movie_id, author, copy, pool).await;
}

#[tokio::main]
async fn main() {
    dotenv::from_filename("/home/gus/movieclublol/.env").unwrap();
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

    let listener = TcpListener::bind("0.0.0.0:80").combine(TcpListener::bind("0.0.0.0:443"));
    let _ = Server::new(listener).run(app).await;
}

#[handler]
async fn index(data: Data<&Pool<Postgres>>) -> Response {
    let pool: &Pool<Postgres> = data.0;
    let movies = movies::list_html(movies::all(pool).await, pool).await;
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
    return Response::builder().status(StatusCode::OK).body(home_html);
}
