use actix_web::{web, App, HttpServer, Responder};

/// From https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/moves", web::get().to(get_moves))
            .route("/moves", web::post().to(add_moves))
            .route("/moves/{id}", web::delete().to(delete_moves))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// curl 127.0.0.1:8080/moves
pub async fn get_moves() -> impl Responder {
    format!("hello from get moves")
}

// curl -X POST 127.0.0.1:8080/users
pub async fn add_moves() -> impl Responder {
    format!("hello from add moves")
}

pub async fn delete_moves() -> impl Responder {
    format!("hello from delete moves")
}
