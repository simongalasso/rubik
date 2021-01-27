use actix_web::{get, post, web::{self, Json, Path}, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[post("/json")]
async fn json(info: Json<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}", info.username))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("req_body: {}", req_body);
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[post("/shuffle")]
async fn shuffle(req_body: String) -> impl Responder {
    let shuffle: String = "D D D D D D'".to_string();
    HttpResponse::Ok().body(shuffle)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print_launch_resume();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(shuffle)
            .service(json)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub fn print_launch_resume() {
    let ascii_name = r#" *******           **      ** **     **        
/**////**         /**     // /**    //*        
/**   /**  **   **/**      **/**  ** /   ******
/*******  /**  /**/****** /**/** **     **//// 
/**///**  /**  /**/**///**/**/****     //***** 
/**  //** /**  /**/**  /**/**/**/**     /////**
/**   //**//******/****** /**/**//**    ****** 
//     //  ////// /////   // //  //    //////  "#;
    eprintln!("{}", ascii_name);
    eprintln!();
    eprintln!("Server listening on:\t{:?}", "127.0.0.1:8080");
    eprintln!();
    eprintln!("Source code:\t\thttps://github.com/sgalasso42/rubik");
    eprintln!("Contact:\t\thttps://github.com/sgalasso42/rubik or bonjour@rubik.com");
    eprintln!();
}