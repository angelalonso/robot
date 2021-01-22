mod api {
    use actix_web::{web, App, HttpServer, Responder};

    #[actix_rt::main]
    async fn run() -> std::io::Result<()> {
        let address = "127.0.0.1:8080";

        std::env::set_var("RUST_LOG", "actix_web=debug");

        // Start http server
        HttpServer::new(move || {
            App::new()
                .route("/moves", web::get().to(self::get_moves))
                .route("/moves", web::post().to(self::add_moves))
                .route("/moves/{id}", web::delete().to(self::delete_moves))
        })
        .bind(address.to_string())?
        .run()
        .await
    }

    // curl 127.0.0.1:8080/moves
    #[allow(dead_code)]
    pub async fn get_moves() -> impl Responder {
        format!("hello from get moves")
    }

    // curl -X POST 127.0.0.1:8080/users
    #[allow(dead_code)]
    pub async fn add_moves() -> impl Responder {
        format!("hello from add moves")
    }

    #[allow(dead_code)]
    pub async fn delete_moves() -> impl Responder {
        format!("hello from delete moves")
    }
}

