use actix_web::{
    get,
    App, HttpResponse, HttpServer, Responder,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world from Rust!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::hello;
    use actix_web::App;

    #[actix_rt::test]
    async fn test() {
        let srv = actix_test::start(|| App::new().service(hello));
        let req = srv.get("/");
        let response = req.send().await.unwrap();
        assert!(response.status().is_success());
    }
}