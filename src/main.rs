use poem::{get, handler, listener::TcpListener, web::Json, web::Path, Route, Server};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    let app = Route::new().at("/movie/:title", get(get_movie));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
