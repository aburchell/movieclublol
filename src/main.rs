use poem::{get, handler, listener::TcpListener, web::Json, web::Path, Route, Server};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Movie {
    id: Uuid,
    title: String,
    director: String,
    year: i32,
    ratings: Vec<Rating>,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Rating {
    id: Uuid,
    movie: Movie,
    score: i32,
    out_of: i32,
    unit: Unit,
    user: User,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct User {
    id: Uuid,
    name: String,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Unit {
    id: Uuid,
    name: String,
}

#[handler]
fn get_movie(Path(id): Path<String>) {
    // -> Json<Movie> {
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().unwrap();
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let movie = sqlx::query!(
        r#"
            INSERT INTO movies (title, director, year )
            VALUES ( $1, $2, $3 )
            RETURNING *
        "#,
        "Past Lives", "Celine Song", 2023
        )
        .fetch_one(&pool)
        .await
        .unwrap();
    println!("{:?}", movie);

    let app = Route::new().at("/movie/:title", get(get_movie));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
