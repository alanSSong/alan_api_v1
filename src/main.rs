use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log4rs;
use log::{info};

#[get("/")]
async fn hello() -> impl Responder {
    info!("booting up 123 {}", 200);
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
     // 加载 log4rs 配置文件
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    HttpServer::new(move || {
        App::new()
            // 不支持写入文件
            // .wrap(Logger::default())
            // .wrap(Logger::new(r#"%a "%U" %s %b "%{Referer}i" "%{User-Agent}i" %Dms"#))
            // .app_data(web::Data::new(file_logger.clone()))
            // .wrap(TracingLogger::new())
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
