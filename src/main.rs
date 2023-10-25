use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, middleware::Logger, web, Result};
use log4rs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct HelloWorld {
    hello: String
}

#[derive(Deserialize, Serialize)]
struct Info {
    username: String
}


#[get("/")]
async fn hello() -> HttpResponse {
    // HttpResponse::Ok().body("Hello world!")
    HttpResponse::Ok().json(HelloWorld {
        hello: "world".to_string(),
    })
}

#[get("/info/{username}")]
async fn search(username: web::Path<String>) -> Result<impl Responder>  {
    let obj = Info {
        username: username.to_string(),
    };
    Ok(web::Json(obj))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
     // 加载 log4rs 配置文件
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(r#"%a "%U" %s %b "%{Referer}i" "%{User-Agent}i" %Dms"#))
            .service(hello)
            .service(search)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
