use dotenv::dotenv;
use poem::{get, handler, listener::TcpListener, web::Data, web::Path, EndpointExt, Route, Server};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::Pool;

mod movies;

// Review handlers
// fn get_review(){}
// fn get_all_reviews_for_movie(Path(movie_id): Path<i32>, data:Data<&Pool<Postgres>>)

// Movie handlers
#[handler]
async fn get_movie(Path(id): Path<i32>, data: Data<&Pool<Postgres>>) -> String {
    let pool: &Pool<Postgres> = data.0;
    let movie = movies::from_id(id, pool).await;
    return movies::movie_html(movie);
}

#[handler]
async fn get_all_movies() {}

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
        // .at("/reviews/:movie_id", get(get_all_reviews_for_movie))
        .at("/movie/:title", get(get_movie))
        .at("/movies", get(get_all_movies))
        .data(pool);

    Server::new(TcpListener::bind("0.0.0.0:300")).run(app).await;
}
