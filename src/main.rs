mod db;

use actix_web::{delete, get, post, put, web, HttpResponse, HttpServer};
use db::DB;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize)]
struct MovieForm {
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rating: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    movie_id: Option<String>,
}

#[get("/movies/{user}")]
async fn get_movies(user: web::Path<String>, db: web::Data<DB>) -> Result<HttpResponse> {
    match db.get_movies(&user).await {
        Ok(movies) => Ok(HttpResponse::Ok().json(movies)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

#[get("/movies/{user}/{id}")]
async fn get_movie(
    path: web::Path<(String, String)>,
    db: web::Data<DB>,
) -> Result<HttpResponse> {
    match db.get_movie(&path.0, &path.1).await {
        Ok(movie) => Ok(HttpResponse::Ok().json(movie)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

#[post("/movies/{user}")]
async fn post_movie(
    user: web::Path<String>,
    movie: web::Form<MovieForm>,
    db: web::Data<DB>,
) -> Result<HttpResponse> {
    match db
        .post_movie(&user, movie.title.clone(), movie.rating)
        .await
    {
        Ok(movie) => Ok(HttpResponse::Ok().json(movie)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

#[put("/movies/{user}/{id}")]
async fn put_movie(
    path: web::Path<(String, String)>,
    movie: web::Form<MovieForm>,
    db: web::Data<DB>,
) -> Result<HttpResponse> {
    match db
        .put_movie(&path.0, &path.1, movie.title.clone(), movie.rating)
        .await
    {
        Ok(movie) => Ok(HttpResponse::Ok().json(movie)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

#[delete("/movies/{user}/{id}")]
async fn delete_movie(
    path: web::Path<(String, String)>,
    db: web::Data<DB>,
) -> Result<HttpResponse> {
    match db.delete_movie(&path.0, &path.1).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        actix_web::App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .service(get_movies)
            .service(get_movie)
            .service(post_movie)
            .service(put_movie)
            .service(delete_movie)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
