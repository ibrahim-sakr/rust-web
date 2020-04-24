use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use listenfd::ListenFd;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // start logger
    env_logger::init();

    let app = || { App::new()
        .wrap(Logger::default()) // register logger as a middleware to log all requests
        .route("/", web::get().to(home_page)) // home page route
        .route("/users", web::get().to(user_page)) // another dummy route
    };

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(app);

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}

async fn home_page() -> impl Responder {
    HttpResponse::Ok().body("this is my new home page")
}

async fn user_page() -> impl Responder {
    HttpResponse::Ok().body("user page")
}
