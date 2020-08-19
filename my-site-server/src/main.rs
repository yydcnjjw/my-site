use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, get};
use listenfd::ListenFd;

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open("../my-site-web/dist/my-site-web/index.html")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(fs::Files::new(
            "/",
            "../my-site-web/dist/my-site-web",
        ))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
