use actix_web::{get, post, web::{self, Json, Path}, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[get("/")]
pub async fn load_html() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./public/welcome.html").to_string())
}

#[post("/json")]
async fn json(info: Json<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}", info.username))
}

#[get("/scramble")]
async fn scramble() -> impl Responder {
    let mut rng = rand::thread_rng();
    let shuffle: String = rng.gen_range(0, 99).to_string();
    HttpResponse::Ok().body(shuffle)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print_launch_resume();
    HttpServer::new(|| {
        App::new()
            .service(load_html)
            .service(scramble)
            .service(json)
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